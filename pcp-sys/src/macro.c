#include <pcp/pmapi.h>
#include <pcp/pmda.h>

pmLongOptions pmda_opt_header() {
  pmLongOptions s = PMDA_OPTIONS_HEADER("Options");
  return s;
}

pmLongOptions pmda_opt_end() {
  pmLongOptions s = PMDA_OPTIONS_END;
  return s;
}

pmLongOptions pmda_opt_debug() {
  pmLongOptions s = PMDAOPT_DEBUG;
  return s;
}

pmLongOptions pmda_opt_domain() {
  pmLongOptions s = PMDAOPT_DOMAIN;
  return s;
}

pmLongOptions pmda_opt_helptext() {
  pmLongOptions s = PMDAOPT_HELPTEXT;
  return s;
}

pmLongOptions pmda_opt_inet() {
  pmLongOptions s = PMDAOPT_INET;
  return s;
}

pmLongOptions pmda_opt_ipv6() {
  pmLongOptions s = PMDAOPT_IPV6;
  return s;
}

pmLongOptions pmda_opt_logfile() {
  pmLongOptions s = PMDAOPT_LOGFILE;
  return s;
}

pmLongOptions pmda_opt_pipe() {
  pmLongOptions s = PMDAOPT_PIPE;
  return s;
}

pmLongOptions pmda_opt_unix() {
  pmLongOptions s = PMDAOPT_UNIX;
  return s;
}

pmLongOptions pmda_opt_username() {
  pmLongOptions s = PMDAOPT_USERNAME;
  return s;
}
