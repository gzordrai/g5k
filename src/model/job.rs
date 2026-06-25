use serde::{Deserialize, Serialize};

use crate::{Collection, Links};

/// Jobs query details option.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum JobDetails {
    Yes,
    No,
}

/// Jobs query paramaters.
///
/// See the [Grid'5000 API documention](https://api.grid5000.fr/doc/stable/#tag/job/paths/~1stable~1sites~1{siteId}~1jobs/get).
#[derive(Debug, Default, Serialize)]
pub struct JobsQuery {
    /// Limit the number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    /// Paginate through the collection with multiple requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,

    /// Filter jobs with a specific queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,

    /// Filter jobs with a specific name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Filter jobs by state (waiting, launching, running, hold, error, terminated), as a comma-separated list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    /// Filter jobs with a specific owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,

    /// Get more details (assigned_nodes and resources_by_types) for each job in the list. Should be 'yes' or 'no'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<JobDetails>,
}

/// A collection of jobs.
pub type Jobs = Collection<Job>;

/// Job submission payload.
///
/// See the [Grid'5000 API documention](https://api.grid5000.fr/doc/stable/#tag/job/paths/~1stable~1sites~1{siteId}~1jobs/post).
#[derive(Debug, Default, Serialize)]
pub struct JobPayload<'a> {
    /// The command to execute when the job starts.
    pub command: &'a str,

    /// A description of the resources you want to book for your job, in OAR format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<&'a str>,

    /// The directory in which the command will be launched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<&'a str>,

    /// The path to the file that will contain the STDERR output of your command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stderr: Option<&'a str>,

    /// The path to the file that will contain the STDOUT output of your command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stdout: Option<&'a str>,

    /// A string containing SQL constraints on the resources (see OAR documentation for more details).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<&'a str>,

    /// Request that the job starts (and optionally ends) at a specified time using the YYYY-MM-DD hh:mm:ss format.
    /// If YYYY-MM-DD is omitted, it defaults to the current day. In hh:mm:ss, ss and mm can be omitted. When an
    /// end date is provided the job walltime is inferred, unless provided in the resources request which prevails.
    /// The special keyword now can be used as the start date to request an advance reservation job that starts
    /// just now.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<&'a str>,

    /// An array of job types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<&'a [&'a str]>,

    /// A project name to link your job to, set by default to the default one specified (if so) in UMS (known as GGA).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<&'a str>,

    /// A job name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,

    /// A job queue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<&'a str>,

    /// Specify a notification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify: Option<&'a str>,
}

/// OAR job states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobState {
    Waiting,
    Launching,
    Running,
    Hold,
    Error,
    Terminated,
    #[serde(other)]
    Unknown,
}

/// OAR job mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum JobMode {
    INTERACTIVE,
    PASSIVE,
}

/// An OAR event attached to a job.
#[derive(Debug, Deserialize)]
pub struct JobEvent {
    pub uid: u64,
    pub created_at: i64,

    #[serde(rename = "type")]
    pub event_type: String,
    pub description: String,
}

/// Resources allocated to an OAR job, grouped by resource type.
#[derive(Debug, Default, Deserialize)]
pub struct ResourcesByTypes {
    #[serde(default)]
    pub cores: Vec<String>,

    #[serde(default)]
    pub vlans: Vec<u32>,

    #[serde(default)]
    pub disks: Vec<String>,

    #[serde(default)]
    pub subnets: Vec<String>,
}

/// An OAR job.
///
/// See the [Grid'5000 API documention](https://api.grid5000.fr/doc/stable/#tag/job/paths/~1stable~1sites~1{siteId}~1jobs~1{jobId}/get).
#[derive(Debug, Deserialize)]
pub struct Job {
    /// The unique identifier of the job.
    pub uid: u64,

    /// The job's owner.
    pub user_uid: String,

    /// The job's owner.
    pub user: String,

    /// The walltime of job, in seconds.
    pub walltime: u64,

    /// The job's queue.
    pub queue: String,

    /// The state of job, can be: waiting, launching, running, hold, error, terminated.
    pub state: JobState,

    /// The job's project.
    pub project: String,

    /// The job's name.
    #[serde(default)]
    pub name: Option<String>,

    /// The OAR job's types.
    pub types: Vec<String>,

    /// The job's mode ('INTERACTIVE or PASSIVE').
    pub mode: JobMode,

    /// The job's command.
    pub command: String,

    /// The job's submission time, as a timestamp.
    pub submitted_at: i64,

    /// The job's start time, as a timestamp.
    pub started_at: i64,

    /// The job's stop time (if already stopped), as a timestamp.
    #[serde(default)]
    pub stopped_at: Option<i64>,

    /// The job's scheduled time (if not already started), as a timestamp.
    #[serde(default)]
    pub scheduled_at: Option<i64>,

    /// Various OAR message.
    pub message: String,

    /// SQL constraints on OAR's resources.
    pub properties: String,

    /// Directory of command launch.
    pub directory: String,

    /// List of OAR events for job (like a kill request, and then the killed by OAR event.
    pub events: Vec<JobEvent>,

    /// List of nodes assigned to job (if any).
    #[serde(default)]
    pub assigned_nodes: Vec<String>,

    /// Assigned resources to job, by type ('cores', 'vlans', subnets, disks).
    #[serde(default)]
    pub resources_by_types: Option<ResourcesByTypes>,
    pub links: Links,
}

/// A tri-state OAR option flag: enabled, disabled, or forbidden for the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OptionFlag {
    No,
    Yes,
    Forbidden,
}

/// A binary OAR option flag: used or not used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BinaryFlag {
    No,
    Yes,
}

/// Walltime-change status for a running job.
#[derive(Debug, Deserialize)]
pub struct WalltimeChange {
    /// The job id.
    pub uid: u64,

    /// The job walltime.
    pub walltime: String,

    /// Possible walltime increase authorized in OAR configuration, can be 'UNLIMITED' or a duration with '0:0:0' format.
    pub possible: String,

    /// Current walltime change request timeout.
    pub timeout: u64,

    /// Describe if current walltime change was made with 'force' option, 'FORBIDDEN' if disabled for current user.
    pub force: OptionFlag,

    /// Describe if current walltime change was made with 'delay_next_jobs' option, 'FORBIDDEN' if disabled for current user.
    pub delay_next_jobs: OptionFlag,

    /// Describe if current walltime change was made with 'whole' option.
    pub whole: BinaryFlag,

    /// Total granted walltime duration change that was made.
    pub granted: String,

    /// Pending walltime duration change.
    pub pending: String,

    /// Granted walltime duration that was made using 'whole' option.
    pub granted_with_whole: String,

    /// Granted walltime duration that was made using 'force' option.
    pub granted_with_force: String,

    /// Granted walltime duration that was made using 'delay_next_jobs' option.
    pub granted_with_delay_next_jobs: String,
    pub links: Links,
}

/// Job walltime change submission payload.
#[derive(Debug, Serialize)]
pub struct WalltimeChangeRequest<'a> {
    /// The new wanted walltime, format is <[+]new walltime>. If no signed is used, the value is absolute.
    pub walltime: &'a str,

    /// Request walltime increase to be trialed or applied immediately regardless of any otherwise configured delay. Must be authorized in OAR configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,

    /// Request walltime increase to possibly delay next batch jobs. Must be authorized in OAR configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_next_jobs: Option<bool>,

    /// Request walltime increase to be trialed or applied wholly at once, or not applied otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whole: Option<bool>,

    /// Specify a timeout (in seconds) after which the walltime change request will be aborted if not already accepted by the scheduler. A default timeout could be set in OAR configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<u64>,
}
