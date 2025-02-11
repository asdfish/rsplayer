use {
    cpal::{
        DeviceNameError,
        DevicesError,
        platform::{
            Device,
            Host,
        },
        traits::{
            DeviceTrait,
            HostTrait,
        },
    },
    crate::flags::Config,
    thiserror::Error,
};

fn get_device<D: DeviceTrait, H: HostTrait<Device = D>>(host: &H, target_device: &'static str) -> Result<Option<D>, DevicesError> {
    Ok(host.output_devices()?
        .find(|device| {
            device.name()
                .inspect_err(|err| eprintln!("Failed to get device name: {}", err))
                .map(|name| name == target_device)
                .unwrap_or_default()
        }))
}

pub struct Player {
    device: Device,
    host: Host,
}
impl Player {
    pub fn new(config: Config) -> Result<Self, DeviceNotFoundError> {
        let host = cpal::default_host();
        let device = config.device.and_then(|device| Some(get_device(&host, device)
            .unwrap_or_else(|err| {
                eprintln!("Failed to get device names: {}", err);
                None
            })
            .ok_or(DeviceNotFoundError(device)))
        )
            .unwrap_or_else(|| {
                host.default_output_device().ok_or(DeviceNotFoundError("default"))
            })?;

        Ok(Self {
            device,
            host,
        })
    }
}

#[derive(Debug, Error)]
#[error("Device `{0}` was not found.")]
pub struct DeviceNotFoundError(&'static str);
