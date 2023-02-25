use once_cell::sync::Lazy;
use pmda::*;
use pmda_sys::*;
use std::ffi::{CString, NulError};
use std::sync::Mutex;

const DOMAIN: u32 = 450;

static COUNTER: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

fn metric_up1() -> pmdaMetric {
    let mut metric = pmdaMetric::default();
    metric.m_desc = pmDesc::default();
    metric.m_desc.pmid = pm_id(0, 0);
    metric.m_desc.type_ = PM_TYPE_U64 as i32;
    metric.m_desc.indom = PM_INDOM_NULL;
    metric.m_desc.sem = PM_SEM_INSTANT as i32;
    metric.m_desc.units = pmUnits::default();
    metric
}

fn metric_up2() -> pmdaMetric {
    let mut metric = pmdaMetric::default();
    metric.m_desc = pmDesc::default();
    metric.m_desc.pmid = pm_id(0, 1);
    metric.m_desc.type_ = PM_TYPE_U64 as i32;
    metric.m_desc.indom = PM_INDOM_NULL;
    metric.m_desc.sem = PM_SEM_INSTANT as i32;
    metric.m_desc.units = pmUnits::default();
    metric
}

fn count_up1(atom: *mut pmAtomValue) -> i32 {
    let mut cnt = COUNTER.lock().unwrap();
    *cnt += 1;

    unsafe { (*atom).ull = *cnt };
    return 0;
}

fn count_up2(atom: *mut pmAtomValue) -> i32 {
    let mut cnt = COUNTER.lock().unwrap();
    *cnt += 2;

    unsafe { (*atom).ull = *cnt };
    return 0;
}

extern "C" fn counter_fetch(metrics: *mut pmdaMetric, inst: u32, atom: *mut pmAtomValue) -> i32 {
    if inst != PM_IN_NULL {
        return PM_ERR_INST;
    }

    let cluster = unsafe { get_pmid_cluster((*metrics).m_desc.pmid) };
    if cluster != 0 {
        return PM_ERR_PMID;
    }

    let item = unsafe { get_pmid_item((*metrics).m_desc.pmid) };
    match item {
        0 => count_up1(atom),
        1 => count_up2(atom),
        _ => PM_ERR_PMID,
    }
}

fn main() -> Result<(), NulError> {
    let pmda = Pmda::new("counter")?;
    pmda.init();

    let mut dispatch = Interface::default();
    pmda.set_daemon(&mut dispatch, DOMAIN);

    let short_options = CString::new("D:d:h:l:U:?")?;
    let mut long_options = vec![
        pmdaopt_header(),
        pmdaopt_debug(),
        pmdaopt_domain(),
        pmdaopt_helptext(),
        pmdaopt_logfile(),
        pmdaopt_username(),
        pmdaopt_end(),
    ];

    let mut options = pmda_options(&short_options, &mut long_options);

    dispatch.get_options(&mut options);
    if options.errors != 0 {
        pmda_print_usage(&mut options);
        return Ok(());
    }

    dispatch.open_log();

    pmda.set_uid();

    dispatch.set_fetch_callback(counter_fetch);

    let mut metrics = vec![metric_up1(), metric_up2()];

    dispatch.init(&mut metrics);

    dispatch.connect();

    dispatch.main();

    Ok(())
}
