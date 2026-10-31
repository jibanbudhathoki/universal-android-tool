use crate::verification::OwnershipVerification;
use uat_shared_types::{DeviceInfo, RecoveryGuide};

pub struct ScreenLockRecoveryWorkflow;

impl ScreenLockRecoveryWorkflow {
    /// Generates manufacturer-specific hardware recovery button instructions and data loss warnings
    pub fn get_official_guidance(device: &DeviceInfo) -> RecoveryGuide {
        let mfg_lower = device.manufacturer.to_lowercase();

        let (title, legal_notice, prerequisites, official_steps, official_support_url) =
            if mfg_lower.contains("samsung") {
                (
                    "Samsung Galaxy Official Screen Lock Reset & Recovery Procedure".to_string(),
                    "Modern Samsung Galaxy smartphones utilize Knox File-Based Encryption (FBE) bound to hardware key storage. Universal Android Tool strictly adheres to security standards and does NOT bypass screen locks. When a lock is forgotten, legitimate recovery requires either Samsung SmartThings Find remote unlock or a factory data reset in Recovery Mode. All local data will be erased.".to_string(),
                    vec![
                        "Ownership verification confirmed".to_string(),
                        "Acknowledgement that all local device data, photos, and messages will be permanently erased".to_string(),
                        "Device battery charge at least 50%".to_string(),
                        "USB cable connected to a powered PC (required for modern One UI recovery entry)".to_string(),
                    ],
                    vec![
                        "Method A (Remote Reset without Cable): Visit https://smartthingsfind.samsung.com or https://android.com/find on another device and select 'Erase Device'.".to_string(),
                        "Method B (Hardware Recovery Mode):".to_string(),
                        "1. Connect the Galaxy phone to a computer with a USB-C cable.".to_string(),
                        "2. Turn off the device completely (or force restart by holding Volume Down + Side key for 7 seconds, then immediately switch to Volume Up + Side key).".to_string(),
                        "3. Hold Volume Up + Side/Power button until the Samsung Galaxy logo appears, then release.".to_string(),
                        "4. In Android Recovery, use Volume Down to highlight 'Wipe data / factory reset' and press the Side key to select.".to_string(),
                        "5. Highlight 'Factory data reset' and confirm. When finished, select 'Reboot system now'.".to_string(),
                    ],
                    "https://www.samsung.com/us/support/answer/ANS00084722/".to_string(),
                )
            } else if mfg_lower.contains("pixel") || mfg_lower.contains("google") {
                (
                    "Google Pixel Official Screen Lock Recovery Procedure".to_string(),
                    "Google Pixel devices feature Titan M2 tamper-resistant hardware security. Universal Android Tool does NOT exploit lock screens. Forgotten PINs, patterns, or passwords require an authorized remote wipe or bootloader recovery reset. All local encryption keys and data are destroyed during reset.".to_string(),
                    vec![
                        "Ownership verification".to_string(),
                        "Data loss disclaimer acknowledged".to_string(),
                        "Battery charge at least 50%".to_string(),
                    ],
                    vec![
                        "Method A: Visit https://android.com/find to remotely reset the Pixel device.".to_string(),
                        "Method B (Hardware Recovery):".to_string(),
                        "1. Turn off the Pixel phone completely.".to_string(),
                        "2. Press and hold Power + Volume Down together until the Fastboot menu appears, then release.".to_string(),
                        "3. Use Volume buttons to select 'Recovery Mode' and press the Power button.".to_string(),
                        "4. When the 'No command' screen appears, hold down the Power button, tap Volume Up once, and release both.".to_string(),
                        "5. Use Volume buttons to highlight 'Wipe data / factory reset' and press Power to confirm.".to_string(),
                    ],
                    "https://support.google.com/pixelphone/answer/4596836".to_string(),
                )
            } else if mfg_lower.contains("xiaomi") || mfg_lower.contains("redmi") || mfg_lower.contains("poco") {
                (
                    format!("{} Official Hardware Recovery & Screen Lock Reset", device.manufacturer),
                    "Xiaomi, Redmi, and POCO devices use encrypted user partitions. Universal Android Tool strictly avoids unsafe unlock exploits. Screen lock recovery requires an authorized factory reset through Mi Recovery or Xiaomi Cloud.".to_string(),
                    vec![
                        "Ownership verification".to_string(),
                        "Understanding that user storage will be wiped".to_string(),
                        "Battery charge at least 50%".to_string(),
                    ],
                    vec![
                        "1. Power off the phone completely.".to_string(),
                        "2. Press and hold Volume Up + Power button simultaneously until the phone vibrates and displays the Mi/Redmi/POCO logo.".to_string(),
                        "3. In the Main Menu, use Volume keys to navigate to 'Wipe Data' and press the Power key to select.".to_string(),
                        "4. Select 'Wipe All Data' and highlight 'Confirm'.".to_string(),
                        "5. Once data wipe completes, select 'Back to Main Menu' and 'Reboot system'.".to_string(),
                    ],
                    "https://www.mi.com/global/support".to_string(),
                )
            } else if mfg_lower.contains("oppo") || mfg_lower.contains("realme") {
                (
                    format!("{} Official Screen Lock Reset via ColorOS Recovery", device.manufacturer),
                    "OPPO and Realme devices protect user credentials using secure hardware enclaves. Authorized screen lock reset requires ColorOS Recovery formatting.".to_string(),
                    vec![
                        "Ownership verification".to_string(),
                        "Data loss acknowledgement".to_string(),
                    ],
                    vec![
                        "1. Turn off the device completely.".to_string(),
                        "2. Hold Volume Down + Power button together until the brand logo appears.".to_string(),
                        "3. Select 'English' as recovery language.".to_string(),
                        "4. Select 'Wipe data' -> 'Format data'.".to_string(),
                        "5. Enter the 4-digit verification code displayed on screen and confirm 'Format'.".to_string(),
                    ],
                    "https://support.oppo.com".to_string(),
                )
            } else if mfg_lower.contains("vivo") {
                (
                    "Vivo Official Screen Lock Reset via Recovery Mode".to_string(),
                    "FuntouchOS and OriginOS devices do not permit lock bypass without full data wipe.".to_string(),
                    vec![
                        "Ownership verification".to_string(),
                        "Data loss acknowledgement".to_string(),
                    ],
                    vec![
                        "1. Turn off the phone.".to_string(),
                        "2. Hold Volume Up + Power button until the Fastboot menu appears.".to_string(),
                        "3. Use Volume Down to select 'Recovery Mode' and press Power.".to_string(),
                        "4. Tap 'Clear data' -> 'Clear all data' and confirm 'OK'.".to_string(),
                    ],
                    "https://www.vivo.com/support".to_string(),
                )
            } else if mfg_lower.contains("motorola") {
                (
                    "Motorola Official Screen Lock Reset via AP Fastboot / Recovery".to_string(),
                    "Motorola devices require a standard hardware recovery reset if screen locks are lost.".to_string(),
                    vec![
                        "Ownership verification".to_string(),
                        "Data loss acknowledgement".to_string(),
                    ],
                    vec![
                        "1. Turn off the phone.".to_string(),
                        "2. Hold Volume Down + Power button until the boot options menu appears.".to_string(),
                        "3. Scroll to 'Recovery Mode' and press Power.".to_string(),
                        "4. At the Android robot, press Power and tap Volume Up once.".to_string(),
                        "5. Select 'Wipe data / factory reset' and confirm.".to_string(),
                    ],
                    "https://motorola-global-portal.custhelp.com".to_string(),
                )
            } else if mfg_lower.contains("oneplus") {
                (
                    "OnePlus Official Screen Lock Reset via OxygenOS Recovery".to_string(),
                    "OxygenOS enforces hardware keystore protection. Data wipe in Recovery Mode is the authorized recovery path.".to_string(),
                    vec![
                        "Ownership verification".to_string(),
                        "Data loss acknowledgement".to_string(),
                    ],
                    vec![
                        "1. Turn off the OnePlus device.".to_string(),
                        "2. Hold Volume Down + Power button together until the OnePlus logo appears.".to_string(),
                        "3. Select language, then select 'Format data' and confirm.".to_string(),
                    ],
                    "https://www.oneplus.com/support".to_string(),
                )
            } else {
                (
                    format!("{} Official Screen Lock Reset Procedure", device.manufacturer),
                    "Screen lock bypass without authorization or data wiping is not supported. Legitimate screen lock recovery requires a verified owner reset through Google Find My Device or an authorized device factory reset via recovery mode. All local data will be erased.".to_string(),
                    vec![
                        "Device ownership verification".to_string(),
                        "Understanding that local user data will be permanently erased".to_string(),
                        "Device battery charge at least 50%".to_string(),
                    ],
                    vec![
                        "Method 1: Use Google 'Find My Device' (https://android.com/find) to remotely erase and reset device lock.".to_string(),
                        "Method 2: Boot into Recovery Mode via hardware key combinations (typically Volume Down or Volume Up + Power) and select 'Wipe data / factory reset'.".to_string(),
                        "Method 3: Visit an authorized service center with original invoice documentation.".to_string(),
                    ],
                    "https://support.google.com/android/answer/7663172".to_string(),
                )
            };

        RecoveryGuide {
            id: format!("screen-lock-guide-{}", device.manufacturer.to_lowercase().replace(' ', "-")),
            manufacturer: device.manufacturer.clone(),
            title,
            category: "screen_lock".to_string(),
            legal_notice,
            prerequisites,
            official_steps,
            official_support_url,
        }
    }

    /// Verifies that data loss acknowledgement and ownership verification are strictly met
    pub fn verify_data_loss_acknowledgement(acknowledged: bool, verification: &OwnershipVerification) -> bool {
        acknowledged && verification.is_authorized_for_recovery()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samsung_screen_lock_guidance() {
        let device = DeviceInfo {
            id: "samsung-test".to_string(),
            vendor_id: 1256,
            product_id: 26720,
            manufacturer: "Samsung".to_string(),
            model: "Galaxy S23".to_string(),
            android_version: Some("14".to_string()),
            build_number: None,
            serial_number: None,
            connection_mode: uat_shared_types::ConnectionMode::NativeUsb,
            supported_protocols: vec![],
            capabilities: Default::default(),
            is_authorized: false,
        };

        let guide = ScreenLockRecoveryWorkflow::get_official_guidance(&device);
        assert!(guide.title.contains("Samsung"));
        assert!(guide.official_steps.iter().any(|s| s.contains("USB-C")));
    }

    #[test]
    fn test_pixel_screen_lock_guidance() {
        let device = DeviceInfo {
            id: "pixel-test".to_string(),
            vendor_id: 6353,
            product_id: 20193,
            manufacturer: "Google Pixel".to_string(),
            model: "Pixel 8".to_string(),
            android_version: Some("14".to_string()),
            build_number: None,
            serial_number: None,
            connection_mode: uat_shared_types::ConnectionMode::NativeUsb,
            supported_protocols: vec![],
            capabilities: Default::default(),
            is_authorized: false,
        };

        let guide = ScreenLockRecoveryWorkflow::get_official_guidance(&device);
        assert!(guide.title.contains("Pixel"));
        assert!(guide.legal_notice.contains("Titan"));
        assert!(guide.official_steps.iter().any(|s| s.contains("Fastboot")));
    }
}
