//! avidemux model — video editor.

#[derive(Debug, Clone)]
pub struct Stream {
    pub type_: String,
    pub codec: String,
}

#[derive(Debug, Clone)]
pub struct Job {
    pub input: String,
    pub output: String,
    pub streams: Vec<Stream>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub jobs: Vec<Job>,
}

impl Default for Model {
    fn default() -> Self { Self { jobs: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_job(&mut self, input: &str, output: &str, streams: Vec<Stream>) -> bool {
        self.jobs.push(Job { input: input.into(), output: output.into(), streams, status: "queued".into() });
        true
    }

    pub fn start(&mut self, index: usize) -> bool {
        if let Some(j) = self.jobs.get_mut(index) { j.status = "running".into(); true } else { false }
    }

    pub fn list(&self) -> &[Job] { &self.jobs }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_start() {
        let mut a = Model::new();
        assert!(a.add_job("in.avi", "out.mp4", vec![Stream { type_: "video".into(), codec: "h264".into() }]));
        assert!(a.start(0));
        assert_eq!(a.list()[0].status, "running");
    }
}
