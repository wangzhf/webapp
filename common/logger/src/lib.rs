use chrono::Local;
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::io::Write;

pub enum Flag {
    Dev,
    Prod,
}

pub fn init(flag: Flag) {
    match flag {
        Flag::Dev => {
            std::env::set_var("RUST_LOG", "info,actix-web=info");
            // 设置日志显示格式
            env_logger::Builder::new()
                .format(|buf, record| {
                    writeln!(
                        buf,
                        "{} [{}] - {}",
                        Local::now().format("%Y-%m-%d %H:%M:%S"),
                        record.level(),
                        record.args()
                    )
                })
                .filter(None, LevelFilter::Info)
                // 打印测试代码中的日志，如果不需要可关闭
                .is_test(true)
                .init();
        }
        Flag::Prod => {
            // log4rs::init_file("log4rs.yml", Default::default()).unwrap();
            let log_file = FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(
                    "{d(%Y-%m-%d %H:%M:%S %Z)} {l} {t} [{T}] {f:40.40} {L} - {m}{n}",
                )))
                .build("log/output.log")
                .unwrap();

            let config = Config::builder()
                .appender(Appender::builder().build("logfile", Box::new(log_file)))
                .build(Root::builder().appender("logfile").build(LevelFilter::Info))
                .unwrap();
            log4rs::init_config(config).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use log::*;

    #[test]
    fn test_debug() {
        init(Flag::Dev);
        debug!("hello");
    }

    #[test]
    fn test_info() {
        init(Flag::Dev);
        info!("hello");
    }

    #[test]
    fn test_warn() {
        init(Flag::Dev);
        warn!("hello");
    }

    #[test]
    fn test_error() {
        init(Flag::Dev);
        error!("hello");
    }

    #[test]
    fn test_trace() {
        init(Flag::Dev);
        trace!("hello");
    }
}
