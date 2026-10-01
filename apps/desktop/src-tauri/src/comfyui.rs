//! B10 provider-specific local HTTP and core-node workflow. No project persistence.
use forge_app_core::visual::{
    AspectIntent, ImageProviderAdapter, ImageProviderCapabilities, VisualGenerationBrief,
    validate_visual_brief,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::time::{Duration, Instant};

pub const IMAGE_LIMIT: usize = 8 * 1024 * 1024;
const JSON_LIMIT: u64 = 2 * 1024 * 1024;
pub const JOB_TIMEOUT: Duration = Duration::from_secs(300);
const NODES: [&str; 6] = [
    "CheckpointLoaderSimple",
    "CLIPTextEncode",
    "EmptyLatentImage",
    "KSampler",
    "VAEDecode",
    "SaveImage",
];
#[derive(Clone, Debug, Serialize)]
pub struct ProviderError {
    pub message: &'static str,
}
fn err(message: &'static str) -> ProviderError {
    ProviderError { message }
}
/// Strict authority parser: localhost is pinned to numeric loopback, never DNS-resolved.
pub fn local_url(input: &str) -> Result<String, ProviderError> {
    let authority = input
        .strip_prefix("http://")
        .ok_or_else(|| err("Use an HTTP loopback address with an explicit port."))?;
    let authority = authority.strip_suffix('/').unwrap_or(authority);
    let (host, port) = authority
        .rsplit_once(':')
        .ok_or_else(|| err("Include a loopback port."))?;
    if port.is_empty()
        || !port.bytes().all(|b| b.is_ascii_digit())
        || port.parse::<u16>().ok().filter(|p| *p > 0).is_none()
    {
        return Err(err("Port must be between 1 and 65535."));
    }
    let host = match host {
        "127.0.0.1" | "localhost" => "127.0.0.1",
        "[::1]" => "[::1]",
        _ => return Err(err("Only local loopback ComfyUI hosts are allowed.")),
    };
    Ok(format!("http://{host}:{}", port.parse::<u16>().unwrap()))
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub checkpoint: String,
    pub seed: u64,
    pub width: u32,
    pub height: u32,
    pub steps: u32,
    pub cfg: f64,
    pub sampler: String,
    pub scheduler: String,
}
impl Settings {
    pub fn validate(&self) -> Result<(), ProviderError> {
        if self.seed > 9_007_199_254_740_991
            || !(256..=1024).contains(&self.width)
            || !(256..=1024).contains(&self.height)
            || !self.width.is_multiple_of(64)
            || !self.height.is_multiple_of(64)
            || self.width * self.height > 786432
            || !(1..=40).contains(&self.steps)
            || !self.cfg.is_finite()
            || !(1.0..=15.0).contains(&self.cfg)
            || !safe_model(&self.checkpoint)
            || !safe_choice(&self.sampler)
            || !safe_choice(&self.scheduler)
        {
            return Err(err(
                "Review checkpoint, seed, resolution, steps, CFG, sampler and scheduler bounds.",
            ));
        }
        Ok(())
    }
}
fn safe_choice(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 128
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}
fn safe_model(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 256
        && !s.chars().any(|c| c.is_control() || ":%?#".contains(c))
        && !s.starts_with(['/', '\\'])
        && s.split(['/', '\\'])
            .all(|p| !p.is_empty() && p != "." && p != "..")
        && (s.ends_with(".safetensors") || s.ends_with(".ckpt"))
}
#[derive(Clone, Debug, Serialize)]
pub struct Discovery {
    pub state: &'static str,
    pub message: &'static str,
    pub server_url: String,
    pub server_version: Option<String>,
    pub checkpoints: Vec<String>,
    pub samplers: Vec<String>,
    pub schedulers: Vec<String>,
    pub missing_nodes: Vec<String>,
}
fn choices(objects: &Value, node: &str, key: &str) -> Result<Vec<String>, ProviderError> {
    let list = objects
        .get(node)
        .and_then(|n| n.pointer(&format!("/input/required/{key}/0")))
        .and_then(Value::as_array)
        .ok_or_else(|| err("ComfyUI core-node choices are incompatible."))?;
    if list.len() > 2048 {
        return Err(err("Too many ComfyUI choices."));
    }
    list.iter()
        .map(|v| {
            v.as_str()
                .filter(|s| s.len() <= 256)
                .map(str::to_owned)
                .ok_or_else(|| err("Invalid ComfyUI choice."))
        })
        .collect()
}
pub fn discovery(url: String, stats: &Value, objects: &Value) -> Result<Discovery, ProviderError> {
    let version = stats
        .pointer("/system/comfyui_version")
        .and_then(Value::as_str)
        .filter(|s| s.len() <= 64 && !s.chars().any(char::is_control))
        .map(str::to_owned);
    let missing_nodes = NODES
        .iter()
        .filter(|n| !objects.get(**n).is_some_and(Value::is_object))
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    if !missing_nodes.is_empty() {
        return Ok(Discovery {
            state: "unavailable",
            message: "Required built-in ComfyUI nodes are missing.",
            server_url: url,
            server_version: version,
            checkpoints: vec![],
            samplers: vec![],
            schedulers: vec![],
            missing_nodes,
        });
    }
    let checkpoints = choices(objects, "CheckpointLoaderSimple", "ckpt_name")?
        .into_iter()
        .filter(|s| safe_model(s))
        .collect::<Vec<_>>();
    let samplers = choices(objects, "KSampler", "sampler_name")?
        .into_iter()
        .filter(|s| safe_choice(s))
        .collect::<Vec<_>>();
    let schedulers = choices(objects, "KSampler", "scheduler")?
        .into_iter()
        .filter(|s| safe_choice(s))
        .collect::<Vec<_>>();
    let ready = !checkpoints.is_empty() && !samplers.is_empty() && !schedulers.is_empty();
    Ok(Discovery {
        state: if ready { "ready" } else { "no_model" },
        message: if ready {
            "Local ComfyUI is ready. Checkpoint architecture compatibility is verified by ComfyUI at execution."
        } else {
            "ComfyUI is available, but no compatible local checkpoint was found."
        },
        server_url: url,
        server_version: version,
        checkpoints,
        samplers,
        schedulers,
        missing_nodes,
    })
}
#[derive(Clone)]
pub struct Request {
    pub workflow: Value,
    pub settings: Settings,
    pub brief_hash: String,
}
pub fn workflow(
    brief: &VisualGenerationBrief,
    settings: &Settings,
) -> Result<Request, ProviderError> {
    validate_visual_brief(brief).map_err(|_| err("Compile a valid Visual Brief first."))?;
    settings.validate()?;
    // Fixed node graph; every source/operator string is an input value only.
    let graph = json!({
        "1":{"class_type":"CheckpointLoaderSimple","inputs":{"ckpt_name":settings.checkpoint}},
        "2":{"class_type":"CLIPTextEncode","inputs":{"text":brief.prompt_text,"clip":["1",1]}},
        "3":{"class_type":"CLIPTextEncode","inputs":{"text":brief.negative_prompt_text,"clip":["1",1]}},
        "4":{"class_type":"EmptyLatentImage","inputs":{"width":settings.width,"height":settings.height,"batch_size":1}},
        "5":{"class_type":"KSampler","inputs":{"model":["1",0],"seed":settings.seed,"steps":settings.steps,"cfg":settings.cfg,"sampler_name":settings.sampler,"scheduler":settings.scheduler,"positive":["2",0],"negative":["3",0],"latent_image":["4",0],"denoise":1.0}},
        "6":{"class_type":"VAEDecode","inputs":{"samples":["5",0],"vae":["1",2]}},
        "7":{"class_type":"SaveImage","inputs":{"images":["6",0],"filename_prefix":"TheForge"}}
    });
    if serde_json::to_vec(&graph)
        .map_err(|_| err("Workflow serialization failed."))?
        .len()
        > 128 * 1024
    {
        return Err(err("Workflow exceeds size limit."));
    }
    Ok(Request {
        workflow: graph,
        settings: settings.clone(),
        brief_hash: brief.brief_hash.clone(),
    })
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OutputDescriptor {
    pub filename: String,
    pub subfolder: String,
    #[serde(rename = "type")]
    pub kind: String,
}
impl OutputDescriptor {
    pub fn validate(&self) -> Result<(), ProviderError> {
        if self.kind != "output"
            || self.filename.is_empty()
            || self.filename.len() > 128
            || !self
                .filename
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-.".contains(&b))
            || self.filename == "."
            || self.filename.contains("..")
            || !self.filename.ends_with(".png")
            || self.subfolder.len() > 256
            || (!self.subfolder.is_empty()
                && (!self.subfolder.split('/').all(|p| {
                    !p.is_empty()
                        && p != "."
                        && p != ".."
                        && p.bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
                })))
        {
            return Err(err("Unsafe ComfyUI output descriptor."));
        }
        Ok(())
    }
    pub fn view_path(&self) -> Result<String, ProviderError> {
        self.validate()?;
        Ok(format!(
            "/view?filename={}&subfolder={}&type=output",
            self.filename, self.subfolder
        ))
    }
}
#[derive(Clone, Debug)]
pub enum History {
    Pending,
    Complete(OutputDescriptor),
}
pub fn parse_history(value: &Value, id: &str) -> Result<History, ProviderError> {
    let Some(job) = value.get(id) else {
        return Ok(History::Pending);
    };
    let status = job
        .get("status")
        .ok_or_else(|| err("ComfyUI history has no execution status."))?;
    if status.get("status_str").and_then(Value::as_str) == Some("error") {
        return Err(err(
            "ComfyUI execution failed. Inspect the local ComfyUI UI for details.",
        ));
    }
    if status.get("completed").and_then(Value::as_bool) != Some(true) {
        return Ok(History::Pending);
    }
    let images = job
        .pointer("/outputs/7/images")
        .and_then(Value::as_array)
        .filter(|a| a.len() == 1)
        .ok_or_else(|| err("Expected one owned SaveImage output."))?;
    let descriptor: OutputDescriptor =
        serde_json::from_value(images[0].clone()).map_err(|_| err("Invalid output descriptor."))?;
    descriptor.validate()?;
    Ok(History::Complete(descriptor))
}
pub fn prompt_id(value: &Value) -> Result<String, ProviderError> {
    if value.get("error").is_some()
        || value
            .get("node_errors")
            .is_some_and(|v| v.as_object().is_none_or(|o| !o.is_empty()))
    {
        return Err(err("ComfyUI rejected the core-node workflow."));
    }
    value
        .get("prompt_id")
        .and_then(Value::as_str)
        .filter(|s| {
            s.len() <= 64
                && !s.is_empty()
                && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        })
        .map(str::to_owned)
        .ok_or_else(|| err("ComfyUI returned an invalid prompt ID."))
}
fn owned_running(queue: &Value, id: &str) -> Result<bool, ProviderError> {
    let running = queue
        .get("queue_running")
        .and_then(Value::as_array)
        .filter(|a| a.len() <= 2048)
        .ok_or_else(|| err("Invalid ComfyUI queue status."))?;
    Ok(running
        .iter()
        .any(|entry| entry.get(1).and_then(Value::as_str) == Some(id)))
}
#[derive(Clone, Debug, Serialize)]
pub struct ResultMetadata {
    pub provider: &'static str,
    pub server_version: Option<String>,
    pub brief_hash: String,
    pub prompt_id: String,
    pub settings: Settings,
    pub output: OutputDescriptor,
    pub mime_type: &'static str,
    pub byte_length: usize,
    pub content_blake3: String,
    pub completed_unix_ms: i64,
    pub warnings: Vec<String>,
}
#[derive(Clone)]
pub struct Job {
    pub id: String,
    pub url: String,
    pub request: Request,
    pub version: Option<String>,
    pub started: Instant,
    pub last_poll: Option<Instant>,
    pub state: &'static str,
    pub result: Option<(ResultMetadata, Vec<u8>)>,
}
#[derive(Clone)]
pub struct ComfyAdapter {
    pub url: String,
    agent: ureq::Agent,
}
impl ComfyAdapter {
    pub fn new(url: &str) -> Result<Self, ProviderError> {
        let url = local_url(url)?;
        let config = ureq::Agent::config_builder()
            .proxy(None)
            .max_redirects(0)
            .http_status_as_error(false)
            .timeout_connect(Some(Duration::from_secs(2)))
            .timeout_global(Some(Duration::from_secs(10)))
            .build();
        Ok(Self {
            url,
            agent: config.into(),
        })
    }
    fn json(&self, path: &str, body: Option<Value>) -> Result<Value, ProviderError> {
        let endpoint = format!("{}{path}", self.url);
        let mut response = if let Some(body) = body {
            let bytes =
                serde_json::to_vec(&body).map_err(|_| err("Workflow serialization failed."))?;
            if bytes.len() > 128 * 1024 {
                return Err(err("Workflow exceeds size limit."));
            }
            self.agent
                .post(&endpoint)
                .header("Content-Type", "application/json")
                .send(bytes.as_slice())
        } else {
            self.agent.get(&endpoint).call()
        }
        .map_err(|_| {
            err("Local ComfyUI is unavailable or timed out. Start ComfyUI, then Check Connection.")
        })?;
        // Redirects are never followed, including loopback redirects.
        if !response.status().is_success() {
            return Err(err(
                "Local ComfyUI rejected the request. Check its UI for details.",
            ));
        }
        let bytes = response
            .body_mut()
            .with_config()
            .limit(JSON_LIMIT)
            .read_to_vec()
            .map_err(|_| err("ComfyUI response exceeded bounds or timed out."))?;
        serde_json::from_slice(&bytes).map_err(|_| err("ComfyUI returned invalid JSON."))
    }
    pub fn discover(&self) -> Result<Discovery, ProviderError> {
        let stats = self.json("/system_stats", None)?;
        // Query only the six required nodes, avoiding huge custom-node registries.
        let mut objects = serde_json::Map::new();
        for node in NODES {
            let response = self.json(&format!("/object_info/{node}"), None)?;
            if let Some(info) = response.get(node) {
                objects.insert(node.into(), info.clone());
            }
        }
        discovery(self.url.clone(), &stats, &Value::Object(objects))
    }
    pub fn check(&self) -> Discovery {
        self.discover().unwrap_or_else(|error| Discovery {
            state: "unavailable",
            message: error.message,
            server_url: self.url.clone(),
            server_version: None,
            checkpoints: vec![],
            samplers: vec![],
            schedulers: vec![],
            missing_nodes: vec![],
        })
    }
    pub fn queue(&self, request: Request, client_id: &str) -> Result<Job, ProviderError> {
        let d = self.discover()?;
        if d.state != "ready"
            || !d.checkpoints.contains(&request.settings.checkpoint)
            || !d.samplers.contains(&request.settings.sampler)
            || !d.schedulers.contains(&request.settings.scheduler)
        {
            return Err(err(
                "Selected model/settings are no longer available. Check Connection again.",
            ));
        }
        self.queue_ready(request, client_id, d)
    }
    pub fn queue_ready(
        &self,
        request: Request,
        client_id: &str,
        d: Discovery,
    ) -> Result<Job, ProviderError> {
        let id = prompt_id(&self.json(
            "/prompt",
            Some(json!({"prompt":request.workflow,"client_id":client_id})),
        )?)?;
        Ok(Job {
            id,
            url: self.url.clone(),
            request,
            version: d.server_version,
            started: Instant::now(),
            last_poll: None,
            state: "queued",
            result: None,
        })
    }
    pub fn poll(&self, job: &mut Job) -> Result<(), ProviderError> {
        if job.url != self.url {
            return Err(err("Job belongs to another local server."));
        }
        if job.result.is_some() || job.state == "failed" {
            return Ok(());
        }
        if job.started.elapsed() > JOB_TIMEOUT {
            job.state = "failed";
            return Err(err(
                "Generation timed out. ComfyUI may continue its owned job; its global queue was left untouched.",
            ));
        }
        if job
            .last_poll
            .is_some_and(|t| t.elapsed() < Duration::from_secs(1))
        {
            return Ok(());
        }
        job.last_poll = Some(Instant::now());
        let history = self.json(&format!("/history/{}", job.id), None)?;
        match parse_history(&history, &job.id)? {
            History::Pending => {
                let queue = self.json("/queue", None)?;
                job.state = if owned_running(&queue, &job.id)? {
                    "running"
                } else {
                    "queued"
                };
            }
            History::Complete(output) => {
                let mut response = self
                    .agent
                    .get(&format!("{}{}", self.url, output.view_path()?))
                    .call()
                    .map_err(|_| err("Image fetch failed or timed out."))?;
                if !response.status().is_success()
                    || response
                        .headers()
                        .get("Content-Type")
                        .and_then(|v| v.to_str().ok())
                        .map(|s| s.split(';').next().unwrap_or("").trim())
                        != Some("image/png")
                {
                    return Err(err("Expected a successful PNG image response."));
                }
                let bytes = response
                    .body_mut()
                    .with_config()
                    .limit(IMAGE_LIMIT as u64)
                    .read_to_vec()
                    .map_err(|_| err("Image response exceeded bounds or timed out."))?;
                validate_png(&bytes, &job.request.settings)?;
                let result=ResultMetadata{provider:"comfyui_local",server_version:job.version.clone(),brief_hash:job.request.brief_hash.clone(),prompt_id:job.id.clone(),settings:job.request.settings.clone(),output,mime_type:"image/png",byte_length:bytes.len(),content_blake3:forge_export::visual_image_hash(&bytes),completed_unix_ms:std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_|err("Clock unavailable."))?.as_millis().try_into().map_err(|_|err("Clock out of range."))?,warnings:vec!["Generic checkpoint workflow: no transparency or identity consistency guarantee; no Entity or Canon mutation.".into()]};
                job.result = Some((result, bytes));
                job.state = "completed";
            }
        }
        Ok(())
    }
}
pub fn validate_png(bytes: &[u8], settings: &Settings) -> Result<(), ProviderError> {
    if bytes.len() < 33
        || bytes.len() > IMAGE_LIMIT
        || !bytes.starts_with(b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR")
        || u32::from_be_bytes(bytes[16..20].try_into().unwrap()) != settings.width
        || u32::from_be_bytes(bytes[20..24].try_into().unwrap()) != settings.height
    {
        return Err(err("Invalid PNG header or unexpected image dimensions."));
    }
    Ok(())
}
impl ImageProviderAdapter for ComfyAdapter {
    type Settings = Settings;
    type Request = Request;
    type Output = Job;
    type Error = ProviderError;
    fn capabilities(&self) -> ImageProviderCapabilities {
        ImageProviderCapabilities {
            aspects: vec![
                AspectIntent::Portrait,
                AspectIntent::Landscape,
                AspectIntent::Square,
            ],
            transparent_background: false,
            reference_images: false,
            seed: true,
            variation: true,
        }
    }
    fn compile_request(
        &self,
        brief: &VisualGenerationBrief,
        settings: &Settings,
    ) -> Result<Request, ProviderError> {
        workflow(brief, settings)
    }
    // Returns the owned queued job; polling is explicit, not a detached background worker.
    fn generate(&self, request: Request) -> Result<Job, ProviderError> {
        self.queue(
            request,
            &format!(
                "TheForge-{}-{}",
                std::process::id(),
                crate::PROJECT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            ),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use forge_app_core::visual::{ArtDirection, VisualEntity, VisualRole, compile_visual_brief};
    use forge_app_core::{CharacterForge, CreatureForge, FactionForge, ItemForge, LocationForge};
    use forge_contracts::{CreativeAxes, RootSeed};
    use std::{
        collections::BTreeMap,
        io::{Read, Write},
        net::TcpListener,
        sync::{Arc, Mutex},
        thread,
    };
    fn settings() -> Settings {
        Settings {
            checkpoint: "test.safetensors".into(),
            seed: 42,
            width: 512,
            height: 512,
            steps: 20,
            cfg: 7.0,
            sampler: "euler".into(),
            scheduler: "normal".into(),
        }
    }
    fn brief() -> VisualGenerationBrief {
        let e = CharacterForge::generate(
            &CharacterForge::new_dna(
                RootSeed::new(42),
                "nordic-myth",
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )
            .unwrap(),
        )
        .unwrap()
        .character;
        compile_visual_brief(
            &VisualEntity::Character(Box::new(e)),
            1,
            None,
            ArtDirection {
                avoid: "oversaturation".into(),
                ..Default::default()
            },
            VisualRole::Portrait,
        )
        .unwrap()
    }
    fn objects(models: Vec<&str>) -> Value {
        json!({"CheckpointLoaderSimple":{"input":{"required":{"ckpt_name":[models]}}},"CLIPTextEncode":{},"EmptyLatentImage":{},"KSampler":{"input":{"required":{"sampler_name":[["euler"]],"scheduler":[["normal"]]}}},"VAEDecode":{},"SaveImage":{}})
    }
    struct Reply {
        status: u16,
        mime: &'static str,
        bytes: Vec<u8>,
        extra: &'static str,
        delay: Duration,
    }
    fn reply(v: Value) -> Reply {
        Reply {
            status: 200,
            mime: "application/json",
            bytes: serde_json::to_vec(&v).unwrap(),
            extra: "",
            delay: Duration::ZERO,
        }
    }
    fn mock(replies: Vec<Reply>) -> (String, Arc<Mutex<Vec<String>>>, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let log = Arc::new(Mutex::new(vec![]));
        let requests = log.clone();
        let thread = thread::spawn(move || {
            for reply in replies {
                let started = Instant::now();
                let mut socket = loop {
                    match listener.accept() {
                        Ok((s, _)) => break s,
                        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                            if started.elapsed() > Duration::from_secs(15) {
                                panic!("Mock request missing");
                            }
                            thread::sleep(Duration::from_millis(5));
                        }
                        Err(e) => panic!("{e}"),
                    }
                };
                socket.set_nonblocking(false).unwrap();
                socket
                    .set_read_timeout(Some(Duration::from_secs(3)))
                    .unwrap();
                let mut bytes = vec![];
                let mut buf = [0; 4096];
                loop {
                    let n = socket.read(&mut buf).unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&buf[..n]);
                    if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                        let headers = String::from_utf8_lossy(&bytes[..end]);
                        let len = headers
                            .lines()
                            .find_map(|line| {
                                line.to_ascii_lowercase()
                                    .strip_prefix("content-length:")
                                    .map(|s| s.trim().parse::<usize>().unwrap())
                            })
                            .unwrap_or(0);
                        if bytes.len() >= end + 4 + len {
                            break;
                        }
                    }
                    assert!(bytes.len() < 256 * 1024);
                }
                requests
                    .lock()
                    .unwrap()
                    .push(String::from_utf8(bytes).unwrap());
                thread::sleep(reply.delay);
                let header = format!(
                    "HTTP/1.1 {} Test\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n{}\r\n",
                    reply.status,
                    reply.mime,
                    reply.bytes.len(),
                    reply.extra
                );
                let _ = socket.write_all(header.as_bytes());
                let _ = socket.write_all(&reply.bytes);
            }
        });
        (url, log, thread)
    }
    fn discovery_replies(models: Vec<&str>) -> Vec<Reply> {
        let all = objects(models);
        let mut replies = vec![reply(json!({"system":{"comfyui_version":"0.22.2"}}))];
        for node in NODES {
            replies.push(reply(json!({node:all[node]})));
        }
        replies
    }
    #[test]
    fn strict_loopback_authority() {
        for input in [
            "http://127.0.0.1:8000",
            "http://localhost:8000/",
            "http://[::1]:8000",
        ] {
            assert!(local_url(input).is_ok());
        }
        for input in [
            "https://localhost:8000",
            "http://192.168.0.1:8000",
            "http://example.com:80",
            "http://127.0.0.2:8000",
            "http://127.1:8000",
            "http://2130706433:8000",
            "http://user@localhost:8000",
            "http://localhost:0",
            "http://localhost:65536",
            "http://localhost:80/path",
            "http://localhost:80?x",
            "http://localhost:80#x",
            "http://localhost:80\\evil",
            "http://localhost:80//",
            "http://localhost:80\n",
        ] {
            assert!(local_url(input).is_err(), "{input}");
        }
        assert_eq!(
            local_url("http://localhost:80").unwrap(),
            "http://127.0.0.1:80"
        );
    }
    #[test]
    fn discovery_ready_no_model_missing_nodes() {
        let stats = json!({"system":{"comfyui_version":"0.22.2"}});
        let d = discovery(
            "http://127.0.0.1:1".into(),
            &stats,
            &objects(vec!["test.safetensors"]),
        )
        .unwrap();
        assert_eq!(d.state, "ready");
        assert_eq!(d.server_version.as_deref(), Some("0.22.2"));
        assert_eq!(
            discovery(d.server_url.clone(), &stats, &objects(vec![]))
                .unwrap()
                .state,
            "no_model"
        );
        assert_eq!(
            discovery(d.server_url, &stats, &json!({}))
                .unwrap()
                .missing_nodes
                .len(),
            6
        );
    }
    #[test]
    fn settings_bounds() {
        let mut s = settings();
        s.seed = u64::MAX;
        assert!(s.validate().is_err());
        s = settings();
        s.width = 4096;
        assert!(s.validate().is_err());
        s = settings();
        s.height = 513;
        assert!(s.validate().is_err());
        s = settings();
        s.width = 1024;
        s.height = 1024;
        assert!(s.validate().is_err());
        s = settings();
        s.steps = 41;
        assert!(s.validate().is_err());
        s = settings();
        s.cfg = f64::NAN;
        assert!(s.validate().is_err());
        s = settings();
        s.checkpoint = "../bad.safetensors".into();
        assert!(s.validate().is_err());
        s = settings();
        s.sampler = "euler\n".into();
        assert!(s.validate().is_err());
    }
    #[test]
    fn deterministic_fixed_workflow_preserves_prompt_and_avoid() {
        let b = brief();
        let a = workflow(&b, &settings()).unwrap();
        assert_eq!(a.workflow, workflow(&b, &settings()).unwrap().workflow);
        assert_eq!(a.workflow.as_object().unwrap().len(), 7);
        assert_eq!(a.workflow["2"]["inputs"]["text"], b.prompt_text);
        assert_eq!(a.workflow["3"]["inputs"]["text"], b.negative_prompt_text);
        assert_eq!(a.workflow["5"]["inputs"]["seed"], 42);
        assert_eq!(a.workflow["7"]["inputs"]["filename_prefix"], "TheForge");
        assert!(
            !serde_json::to_string(&a.workflow)
                .unwrap()
                .contains(&b.brief_hash)
        );
    }
    #[test]
    fn five_family_briefs_map_without_domain_mutation() {
        let seed = RootSeed::new(45);
        let axes = CreativeAxes::neutral();
        let entities = vec![
            (
                VisualEntity::Character(Box::new(
                    CharacterForge::generate(
                        &CharacterForge::new_dna(
                            seed,
                            "nordic-myth",
                            axes.clone(),
                            BTreeMap::new(),
                        )
                        .unwrap(),
                    )
                    .unwrap()
                    .character,
                )),
                VisualRole::Portrait,
            ),
            (
                VisualEntity::Location(Box::new(
                    LocationForge::generate(
                        &LocationForge::new_dna(seed, "nordic-myth", axes.clone()).unwrap(),
                    )
                    .unwrap()
                    .location,
                )),
                VisualRole::EstablishingShot,
            ),
            (
                VisualEntity::Faction(Box::new(
                    FactionForge::generate(
                        &FactionForge::new_dna(seed, "nordic-myth", axes.clone()).unwrap(),
                    )
                    .unwrap()
                    .faction,
                )),
                VisualRole::Emblem,
            ),
            (
                VisualEntity::Item(Box::new(
                    ItemForge::generate(
                        &ItemForge::new_dna(seed, "nordic-myth", axes.clone()).unwrap(),
                    )
                    .unwrap()
                    .item,
                )),
                VisualRole::ItemConcept,
            ),
            (
                VisualEntity::Creature(Box::new(
                    CreatureForge::generate(
                        &CreatureForge::new_dna(seed, "nordic-myth", axes).unwrap(),
                    )
                    .unwrap()
                    .creature,
                )),
                VisualRole::CreatureConcept,
            ),
        ];
        for (entity, role) in entities {
            let before = entity.clone();
            let b = compile_visual_brief(&entity, 1, None, ArtDirection::default(), role).unwrap();
            assert_eq!(
                workflow(&b, &settings()).unwrap().workflow["2"]["inputs"]["text"],
                b.prompt_text
            );
            assert_eq!(entity, before);
        }
    }
    #[test]
    fn prompt_text_is_data_not_graph() {
        let mut b = brief();
        b.context.art_direction.medium_style = "\"},\"99\":{\"class_type\":\"EvilNode\"}".into();
        let source = CharacterForge::generate(
            &CharacterForge::new_dna(
                RootSeed::new(42),
                "nordic-myth",
                CreativeAxes::neutral(),
                BTreeMap::new(),
            )
            .unwrap(),
        )
        .unwrap()
        .character;
        let b = compile_visual_brief(
            &VisualEntity::Character(Box::new(source)),
            1,
            None,
            b.context.art_direction,
            VisualRole::Portrait,
        )
        .unwrap();
        let graph = workflow(&b, &settings()).unwrap().workflow;
        assert_eq!(graph.as_object().unwrap().len(), 7);
        assert!(graph.get("99").is_none());
        assert!(
            graph["2"]["inputs"]["text"]
                .as_str()
                .unwrap()
                .contains("EvilNode")
        );
    }
    #[test]
    fn owned_history_and_errors() {
        assert!(matches!(
            parse_history(&json!({"other":{"status":{"completed":true}}}), "owned").unwrap(),
            History::Pending
        ));
        assert!(
            parse_history(
                &json!({"owned":{"status":{"status_str":"error","completed":false}}}),
                "owned"
            )
            .is_err()
        );
        assert!(prompt_id(&json!({"prompt_id":"../bad"})).is_err());
        assert!(prompt_id(&json!({"error":{"message":"PRIVATE"}})).is_err());
        assert_eq!(
            prompt_id(&json!({"prompt_id":"owned-id","node_errors":{}})).unwrap(),
            "owned-id"
        );
    }
    #[test]
    fn descriptors_block_arbitrary_reads() {
        for (filename, subfolder, kind) in [
            ("../x.png", "", "output"),
            ("x.png", "../secret", "output"),
            ("x.png", "", "input"),
            ("x.png?x", "", "output"),
            ("x.png", "C:/secret", "output"),
            ("x.svg", "", "output"),
            ("x.png", "x\\y", "output"),
        ] {
            assert!(
                OutputDescriptor {
                    filename: filename.into(),
                    subfolder: subfolder.into(),
                    kind: kind.into()
                }
                .view_path()
                .is_err()
            );
        }
        assert_eq!(
            OutputDescriptor {
                filename: "TheForge_00001_.png".into(),
                subfolder: "".into(),
                kind: "output".into()
            }
            .view_path()
            .unwrap(),
            "/view?filename=TheForge_00001_.png&subfolder=&type=output"
        );
    }
    #[test]
    fn complete_http_owned_job_png_and_hash() {
        let mut replies = discovery_replies(vec!["test.safetensors"]);
        replies.push(reply(json!({"prompt_id":"owned-id","node_errors":{}})));
        replies.push(reply(json!({})));
        replies.push(reply(
            json!({"queue_running":[[1,"owned-id"]],"queue_pending":[]}),
        ));
        replies.push(reply(json!({"owned-id":{"status":{"status_str":"success","completed":true},"outputs":{"7":{"images":[{"filename":"TheForge_00001_.png","subfolder":"","type":"output"}]}}}})));
        let png = include_bytes!("../tests/fixtures/b10.png").to_vec();
        replies.push(Reply {
            mime: "image/png",
            bytes: png.clone(),
            ..reply(json!({}))
        });
        let (url, log, server) = mock(replies);
        let adapter = ComfyAdapter::new(&url).unwrap();
        let mut job = adapter
            .queue(workflow(&brief(), &settings()).unwrap(), "unique-client")
            .unwrap();
        assert_eq!(job.id, "owned-id");
        adapter.poll(&mut job).unwrap();
        assert_eq!(job.state, "running");
        job.last_poll = None;
        adapter.poll(&mut job).unwrap();
        assert_eq!(job.state, "completed");
        let (result, bytes) = job.result.unwrap();
        assert_eq!(bytes, png);
        assert_eq!(
            result.content_blake3,
            forge_export::visual_image_hash(&bytes)
        );
        assert_eq!(result.settings.seed, 42);
        server.join().unwrap();
        let requests = log.lock().unwrap();
        assert_eq!(requests.len(), 12);
        assert!(requests[7].contains("unique-client"));
        assert!(requests[8].starts_with("GET /history/owned-id "));
        assert!(
            requests[11]
                .starts_with("GET /view?filename=TheForge_00001_.png&subfolder=&type=output ")
        );
        for request in requests.iter() {
            assert!(
                !["/interrupt", "/free", "POST /queue", "POST /history"]
                    .iter()
                    .any(|bad| request.contains(bad))
            );
        }
    }
    #[test]
    fn mock_http_no_models() {
        let (url, _, server) = mock(discovery_replies(vec![]));
        assert_eq!(ComfyAdapter::new(&url).unwrap().check().state, "no_model");
        server.join().unwrap();
    }
    #[test]
    fn redirects_never_followed() {
        let (url, log, server) = mock(vec![Reply {
            status: 302,
            extra: "Location: http://192.168.1.5:80/secret\r\n",
            ..reply(json!({}))
        }]);
        assert_eq!(
            ComfyAdapter::new(&url).unwrap().check().state,
            "unavailable"
        );
        server.join().unwrap();
        assert_eq!(log.lock().unwrap().len(), 1);
    }
    #[test]
    fn response_bounds_and_private_error_translation() {
        let (url, _, server) = mock(vec![Reply {
            bytes: vec![b'x'; JSON_LIMIT as usize + 1],
            ..reply(json!({}))
        }]);
        let d = ComfyAdapter::new(&url).unwrap().check();
        assert_eq!(d.state, "unavailable");
        assert!(d.message.contains("bounds"));
        server.join().unwrap();
        let (url, _, server) = mock(vec![Reply {
            status: 400,
            ..reply(json!({"error":"PRIVATE PROSE"}))
        }]);
        let d = ComfyAdapter::new(&url).unwrap().check();
        assert!(!d.message.contains("PRIVATE"));
        server.join().unwrap();
    }
    #[test]
    fn png_and_job_timeout_bounds() {
        let png = include_bytes!("../tests/fixtures/b10.png");
        assert!(validate_png(png, &settings()).is_ok());
        assert!(validate_png(b"<svg>", &settings()).is_err());
        let mut s = settings();
        s.width = 768;
        assert!(validate_png(png, &s).is_err());
        let adapter = ComfyAdapter::new("http://127.0.0.1:1").unwrap();
        let mut job = Job {
            id: "owned".into(),
            url: adapter.url.clone(),
            request: workflow(&brief(), &settings()).unwrap(),
            version: None,
            started: Instant::now() - JOB_TIMEOUT - Duration::from_secs(1),
            last_poll: None,
            state: "queued",
            result: None,
        };
        assert!(adapter.poll(&mut job).is_err());
        assert_eq!(job.state, "failed");
    }
    #[test]
    fn running_queue_matches_only_owned_id() {
        assert!(owned_running(&json!({"queue_running":[[1,"owned"]]}), "owned").unwrap());
        assert!(!owned_running(&json!({"queue_running":[[1,"other"]]}), "owned").unwrap());
        assert!(owned_running(&json!({}), "owned").is_err());
    }
    #[test]
    fn image_http_mime_and_size_limits() {
        for (mime, bytes) in [
            ("image/svg+xml", b"<svg/>".to_vec()),
            ("image/png", vec![0; IMAGE_LIMIT + 1]),
        ] {
            let replies = vec![
                reply(
                    json!({"owned":{"status":{"completed":true,"status_str":"success"},"outputs":{"7":{"images":[{"filename":"TheForge_1.png","subfolder":"","type":"output"}]}}}}),
                ),
                Reply {
                    mime,
                    bytes,
                    ..reply(json!({}))
                },
            ];
            let (url, _, server) = mock(replies);
            let adapter = ComfyAdapter::new(&url).unwrap();
            let mut job = Job {
                id: "owned".into(),
                url,
                request: workflow(&brief(), &settings()).unwrap(),
                version: None,
                started: Instant::now(),
                last_poll: None,
                state: "queued",
                result: None,
            };
            assert!(adapter.poll(&mut job).is_err());
            assert!(job.result.is_none());
            server.join().unwrap();
        }
    }
    #[test]
    fn offline_error_and_read_timeout() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        drop(listener);
        assert_eq!(
            ComfyAdapter::new(&format!("http://{address}"))
                .unwrap()
                .check()
                .state,
            "unavailable"
        );
        let (url, _, server) = mock(vec![Reply {
            delay: Duration::from_millis(150),
            ..reply(json!({}))
        }]);
        let mut adapter = ComfyAdapter::new(&url).unwrap();
        adapter.agent = ureq::Agent::config_builder()
            .proxy(None)
            .max_redirects(0)
            .timeout_global(Some(Duration::from_millis(40)))
            .build()
            .into();
        assert_eq!(adapter.check().state, "unavailable");
        server.join().unwrap();
    }
}
