struct StatusReport;
struct Commmit;
struct LogFormat;
struct FileDiff;

pub fn status(status_report: StatusReport) {}
pub fn log(commits: &[Commmit], log_format: LogFormat) {}
pub fn diff(file_diff: &[FileDiff]) {}
