use chrono::{Local, MappedLocalTime, NaiveDate, NaiveDateTime, NaiveTime};
use std::path::PathBuf;

pub fn convert_to_rfc822(date_string: &str, date_format: String) -> String {
    log::info!(
        "Converting date string to RFC822 format : {}, format : {}",
        date_string,
        date_format
    );
    let naive_datetime: NaiveDateTime =
        match NaiveDate::parse_from_str(date_string, date_format.as_str()) {
            //converting a simple date into datetime
            Ok(date) => date.and_time(NaiveTime::default()),
            Err(e) => panic!("Error parsing date string for rss: {}", e),
        };

    let datetime = match naive_datetime.and_local_timezone(Local) {
        MappedLocalTime::Single(dt) => dt,
        MappedLocalTime::Ambiguous(dt0, dt1) => todo!(),
        MappedLocalTime::None => panic!("invalid date/time"),
    };

    let rfc822_format = "%a, %d %b %Y %H:%M:%S %z";
    return datetime.format(rfc822_format).to_string();
}

pub fn decide_static_rss_render_path(
    local_render_env: &crate::RenderEnv,
    requested_rss_serve_path: &str,
) -> String {
    let path_buf = PathBuf::from(requested_rss_serve_path);
    let clean_path = requested_rss_serve_path
        .trim()
        .trim_end_matches("/")
        .trim_start_matches("/");

    if path_buf.is_dir() {
        log::info!("Error: requested path is a directory, not a file.");
        let fqd = format!("{}/{}", local_render_env.static_base, clean_path);
        let fqp = format!("{}/{}/index.xml", local_render_env.static_base, clean_path);
        match std::fs::read_to_string(&fqp) {
            Ok(_) => {
                log::warn!(
                    "Multiple Static rss renders are conflicting for path : {}",
                    fqd
                )
            }
            _ => {}
        }
        match std::fs::create_dir_all(fqd) {
            Ok(_) => {}
            Err(e) => {
                panic!("Fs error: {}", e)
            }
        }
        fqp
    } else {
        log::info!("Requested path is a file directly");
        let fqp = format!("{}/{}", local_render_env.static_base, clean_path);
        let fqp_pathbuf = PathBuf::from(&fqp);
        match std::fs::read_to_string(&fqp) {
            Ok(_) => {
                log::warn!(
                    "Multiple Static rss renders are conflicting for path : {}",
                    fqp
                )
            }
            _ => {}
        }
        match std::fs::create_dir_all(fqp_pathbuf.parent().unwrap()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Fs error: {}", e)
            }
        }
        fqp
    }
}
