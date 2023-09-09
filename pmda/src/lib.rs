use pmda_sys::*;
use std::env;
use std::error::Error;
use std::ffi::{CStr, CString, NulError};
use std::path::PathBuf;
use std::ptr::null_mut;

pub struct Pmda {
    prog_name: CString,
    log_file: CString,
    help_path: CString,
}

impl Pmda {
    pub fn new(prog_name: &str) -> Result<Self, NulError> {
        let log_file = format!("{}.log", prog_name);

        let mut help_path = PathBuf::from(get_config("PCP_PMDAS_DIR").unwrap());
        help_path.push(prog_name);
        help_path.push("help");

        Ok(Pmda {
            prog_name: CString::new(prog_name)?,
            log_file: CString::new(log_file)?,
            help_path: CString::new(help_path.to_str().unwrap())?,
        })
    }

    pub fn init(&self) {
        set_progname(&self.prog_name);
    }

    pub fn set_daemon(&self, dispatch: &mut Interface, domain: u32) {
        dispatch.set_daemon(
            PMDA_INTERFACE_2,
            &self.prog_name,
            domain,
            &self.log_file,
            &self.help_path,
        )
    }

    pub fn set_uid(&self) {
        let username = get_username();
        set_process_identity(username);
    }
}

#[derive(Default)]
pub struct Interface {
    dispatch: pmdaInterface,
}

impl Interface {
    pub fn connect(&mut self) {
        unsafe { pmdaConnect(&mut self.dispatch) };
    }

    pub fn get_options(&mut self, opts: &mut pmdaOptions) -> i32 {
        let args = env::args()
            .map(|a| CString::new(a).unwrap())
            .collect::<Vec<CString>>();
        let argv = args
            .iter()
            .map(|c| c.as_ptr() as *mut i8)
            .collect::<Vec<_>>();

        unsafe { pmdaGetOptions(args.len() as i32, argv.as_ptr(), opts, &mut self.dispatch) }
    }

    pub fn init(&mut self, metrics: &mut [pmdaMetric]) {
        unsafe {
            pmdaInit(
                &mut self.dispatch,
                null_mut(),
                0,
                metrics.as_mut_ptr(),
                metrics.len() as i32,
            )
        };
    }

    pub fn main(&mut self) {
        unsafe { pmdaMain(&mut self.dispatch) };
    }

    pub fn open_log(&mut self) {
        unsafe { pmdaOpenLog(&mut self.dispatch) };
    }

    pub fn set_daemon(
        &mut self,
        interface: u32,
        name: &CStr,
        domain: u32,
        logfile: &CStr,
        help_text: &CStr,
    ) {
        unsafe {
            pmdaDaemon(
                &mut self.dispatch,
                interface as i32,
                name.as_ptr(),
                domain as i32,
                logfile.as_ptr(),
                help_text.as_ptr(),
            )
        };
    }

    pub fn set_fetch_callback(
        &mut self,
        callback: Option<unsafe extern "C" fn(*mut pmdaMetric, u32, *mut pmAtomValue) -> i32>,
    ) {
        unsafe { pmdaSetFetchCallBack(&mut self.dispatch, callback) };
    }
}

pub struct Id {
    inner: *mut pmID,
}

impl Id {
    pub fn new(id: *mut pmID) -> Self {
        Id { inner: id }
    }

    pub fn cluster(&self) -> u32 {
        unsafe { get_pmid_cluster(*self.inner) }
    }

    pub fn item(&self) -> u32 {
        unsafe { get_pmid_item(*self.inner) }
    }
}

pub struct Desc {
    inner: *mut pmDesc,
}

impl Desc {
    pub fn new(desc: *mut pmDesc) -> Self {
        Desc { inner: desc }
    }

    pub fn id(&self) -> Id {
        unsafe { Id::new(&mut (*self.inner).pmid) }
    }
}

pub struct Metric {
    inner: *mut pmdaMetric,
}

impl Metric {
    pub fn new(metric: *mut pmdaMetric) -> Self {
        Metric { inner: metric }
    }

    pub fn desc(&self) -> Desc {
        unsafe { Desc::new(&mut (*self.inner).m_desc) }
    }
}

pub struct AtomValue {
    inner: *mut pmAtomValue,
}

impl AtomValue {
    pub fn new(atom: *mut pmAtomValue) -> Self {
        AtomValue { inner: atom }
    }

    pub fn set_i32(&mut self, value: i32) {
        unsafe { (*self.inner).l = value };
    }

    pub fn set_u32(&mut self, value: u32) {
        unsafe { (*self.inner).ul = value };
    }

    pub fn set_i64(&mut self, value: i64) {
        unsafe { (*self.inner).ll = value };
    }

    pub fn set_u64(&mut self, value: u64) {
        unsafe { (*self.inner).ull = value };
    }

    pub fn set_f32(&mut self, value: f32) {
        unsafe { (*self.inner).f = value };
    }

    pub fn set_f64(&mut self, value: f64) {
        unsafe { (*self.inner).d = value };
    }
}

pub fn pmdaopt() -> &'static CStr {
    let buffer = unsafe { pmda_opt() };
    unsafe { CStr::from_ptr(buffer) }
}

pub fn pmdaopt_header() -> pmLongOptions {
    unsafe { pmda_opt_header() }
}

pub fn pmdaopt_end() -> pmLongOptions {
    unsafe { pmda_opt_end() }
}

pub fn pmdaopt_debug() -> pmLongOptions {
    unsafe { pmda_opt_debug() }
}

pub fn pmdaopt_domain() -> pmLongOptions {
    unsafe { pmda_opt_domain() }
}

pub fn pmdaopt_helptext() -> pmLongOptions {
    unsafe { pmda_opt_helptext() }
}

pub fn pmdaopt_inet() -> pmLongOptions {
    unsafe { pmda_opt_inet() }
}

pub fn pmdaopt_ipv6() -> pmLongOptions {
    unsafe { pmda_opt_ipv6() }
}

pub fn pmdaopt_logfile() -> pmLongOptions {
    unsafe { pmda_opt_logfile() }
}

pub fn pmdaopt_pipe() -> pmLongOptions {
    unsafe { pmda_opt_pipe() }
}

pub fn pmdaopt_unix() -> pmLongOptions {
    unsafe { pmda_opt_unix() }
}

pub fn pmdaopt_username() -> pmLongOptions {
    unsafe { pmda_opt_username() }
}

pub fn pmda_options(short_options: &CStr, long_options: &mut [pmLongOptions]) -> pmdaOptions {
    pmdaOptions {
        short_options: short_options.as_ptr(),
        long_options: long_options.as_mut_ptr(),
        ..Default::default()
    }
}

pub fn pmda_print_usage(options: &mut pmdaOptions) {
    unsafe { pmdaUsageMessage(options) };
}

pub fn pm_id(cluster: u32, item: u32) -> u32 {
    unsafe { pmda_pmid(cluster, item) }
}

fn get_config(name: &str) -> Result<String, Box<dyn Error>> {
    let raw = CString::new(name)?;
    let buffer = unsafe { pmGetConfig(raw.as_ptr()) };

    let value = unsafe { CStr::from_ptr(buffer) };
    Ok(value.to_owned().into_string()?)
}

fn get_pmid_cluster(id: pmID) -> u32 {
    unsafe { pmID_cluster(id) }
}

fn get_pmid_item(id: pmID) -> u32 {
    unsafe { pmID_item(id) }
}

fn get_username() -> &'static CStr {
    let mut buffer = null_mut();
    unsafe { pmGetUsername(&mut buffer) };

    unsafe { CStr::from_ptr(buffer) }
}

fn set_process_identity(username: &CStr) {
    unsafe { pmSetProcessIdentity(username.as_ptr()) };
}

fn set_progname(prog_name: &CStr) {
    unsafe { pmSetProgname(prog_name.as_ptr()) };
}

#[cfg(test)]
mod tests {
    use super::*;

    extern "C" fn test_fetch(_: *mut pmdaMetric, _: u32, _: *mut pmAtomValue) -> i32 {
        0
    }

    #[test]
    fn test_pmda() {
        let pmda = Pmda::new("test").unwrap();
        pmda.init();

        let mut dispatch = Interface::default();
        pmda.set_daemon(&mut dispatch, 510);

        dispatch.open_log();

        dispatch.set_fetch_callback(Some(test_fetch));

        let metric = pmdaMetric {
            m_desc: pmDesc {
                pmid: pmID::default(),
                type_: PM_TYPE_U64 as i32,
                indom: PM_INDOM_NULL,
                sem: PM_SEM_INSTANT as i32,
                units: pmUnits::default(),
            },
            ..Default::default()
        };

        let mut metrics = vec![metric];

        dispatch.init(&mut metrics);
    }
}
