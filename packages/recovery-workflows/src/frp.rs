use crate::verification::OwnershipVerification;
use uat_shared_types::{DeviceInfo, RecoveryGuide};

pub struct FrpRecoveryWorkflow;

impl FrpRecoveryWorkflow {
    /// Generates official, manufacturer-specific FRP recovery guidance
    pub fn get_official_guidance(device: &DeviceInfo) -> RecoveryGuide {
        let mfg_lower = device.manufacturer.to_lowercase();

        let (title, legal_notice, prerequisites, official_steps, official_support_url) =
            if mfg_lower.contains("samsung") {
                (
                    "Samsung Official FRP Recovery & Account Verification Procedure".to_string(),
                    "Factory Reset Protection (FRP) is an intentional Google & Samsung Knox security safeguard. Universal Android Tool strictly adheres to manufacturer security guidelines and does NOT circumvent FRP. Legitimate recovery requires the original owner's Samsung/Google Account or authorized Samsung Service Center verification.".to_string(),
                    vec![
                        "Original invoice or carrier sales receipt with matching device IMEI/serial".to_string(),
                        "Government-issued photo identification matching customer records".to_string(),
                        "Access to the primary Google or Samsung Account credentials".to_string(),
                        "Active Wi-Fi connection on the Galaxy device".to_string(),
                    ],
                    vec![
                        "Step 1: Connect the device to Wi-Fi on the initial setup screen.".to_string(),
                        "Step 2: Enter the original Google Account or Samsung Account associated with this Galaxy device.".to_string(),
                        "Step 3: If password is forgotten, visit https://account.samsung.com or https://accounts.google.com/signin/recovery on another device.".to_string(),
                        "Step 4: For Samsung Knox enterprise-managed devices, contact your IT administrator to release the device from Knox Mobile Enrollment (KME).".to_string(),
                        "Step 5: If the account cannot be recovered, present proof of purchase at an authorized Samsung Service Center for factory re-provisioning.".to_string(),
                    ],
                    "https://www.samsung.com/us/support/troubleshooting/TSG01001402/".to_string(),
                )
            } else if mfg_lower.contains("pixel") || mfg_lower.contains("google") {
                (
                    "Google Pixel Official Account Recovery Procedure".to_string(),
                    "Google Titan M2 Security & Factory Reset Protection (FRP) cannot and should not be bypassed without owner credentials. Universal Android Tool provides verified recovery steps strictly through official Google services.".to_string(),
                    vec![
                        "Original proof of purchase or Google Store order confirmation".to_string(),
                        "Access to recovery email or phone number for the Google Account".to_string(),
                        "Internet connectivity".to_string(),
                    ],
                    vec![
                        "Step 1: On the setup screen, enter the Google Account and password originally registered on the Pixel.".to_string(),
                        "Step 2: If locked out, complete Google Account Recovery at https://accounts.google.com/signin/recovery using backup 2FA methods.".to_string(),
                        "Step 3: If you need to re-flash authorized stock firmware, use the official WebUSB Android Flash Tool at https://flash.android.com.".to_string(),
                        "Step 4: Contact Google Pixel Support directly if account recovery fails with verified proof of ownership.".to_string(),
                    ],
                    "https://support.google.com/pixelphone/answer/4596836".to_string(),
                )
            } else if mfg_lower.contains("xiaomi") || mfg_lower.contains("redmi") || mfg_lower.contains("poco") {
                (
                    format!("{} Official Mi Account & FRP Recovery Procedure", device.manufacturer),
                    "Xiaomi, Redmi, and POCO devices are secured with both Google FRP and Xiaomi Cloud Lock. Universal Android Tool does NOT crack account locks. Authorized recovery requires verified Mi Account or Google Account authentication.".to_string(),
                    vec![
                        "Device IMEI and purchase documentation".to_string(),
                        "Phone number or email linked to the Xiaomi Account".to_string(),
                    ],
                    vec![
                        "Step 1: Enter the Google Account credentials on the FRP screen or Mi Account password on the 'Activate this device' screen.".to_string(),
                        "Step 2: If Mi Account credentials are forgotten, unlock online at https://i.mi.com using SMS or email verification.".to_string(),
                        "Step 3: If Google Account credentials are forgotten, follow the official recovery flow at https://accounts.google.com/signin/recovery.".to_string(),
                        "Step 4: Visit an authorized Xiaomi Service Center with original box, invoice, and photo ID for factory service.".to_string(),
                    ],
                    "https://www.mi.com/global/support".to_string(),
                )
            } else if mfg_lower.contains("oppo") || mfg_lower.contains("realme") {
                (
                    format!("{} Official Account Recovery & Ownership Verification", device.manufacturer),
                    "ColorOS and realme UI incorporate Google FRP and HeyTap Account security. Universal Android Tool only guides through manufacturer-authorized procedures.".to_string(),
                    vec![
                        "Original commercial invoice showing device IMEI".to_string(),
                        "HeyTap / OPPO / Realme account credentials or linked mobile number".to_string(),
                    ],
                    vec![
                        "Step 1: Sign in with the Google Account or HeyTap Account previously linked to the phone.".to_string(),
                        "Step 2: Recover forgotten passwords at https://id.heytap.com.".to_string(),
                        "Step 3: If online recovery is not possible, submit an ownership claim at an authorized service center with original invoice.".to_string(),
                    ],
                    "https://support.oppo.com".to_string(),
                )
            } else if mfg_lower.contains("vivo") {
                (
                    "Vivo Official Account Verification & FRP Guidance".to_string(),
                    "FuntouchOS and OriginOS devices are protected by Google FRP and Vivo Cloud Security. All operations must follow official Vivo customer care protocols.".to_string(),
                    vec![
                        "Official sales invoice showing device serial/IMEI".to_string(),
                        "Registered Vivo Account or Google Account credentials".to_string(),
                    ],
                    vec![
                        "Step 1: Connect to Wi-Fi and verify the Google Account or Vivo Account.".to_string(),
                        "Step 2: Reset forgotten passwords at https://cloud.vivo.com.".to_string(),
                        "Step 3: Contact Vivo Customer Care or visit an authorized service center with valid proof of purchase.".to_string(),
                    ],
                    "https://www.vivo.com/support".to_string(),
                )
            } else if mfg_lower.contains("motorola") {
                (
                    "Motorola Official Account Recovery via Rescue and Smart Assistant".to_string(),
                    "Motorola devices require verified Google Account recovery. Universal Android Tool facilitates authorized firmware restoration using Lenovo LMSA tools.".to_string(),
                    vec![
                        "Proof of purchase".to_string(),
                        "Google Account credentials or Lenovo ID".to_string(),
                    ],
                    vec![
                        "Step 1: Sign in with the original Google Account credentials.".to_string(),
                        "Step 2: For firmware restoration, download the official Lenovo Rescue and Smart Assistant (LMSA) from https://support.lenovo.com.".to_string(),
                        "Step 3: Contact Motorola Global Support if credentials cannot be recovered.".to_string(),
                    ],
                    "https://motorola-global-portal.custhelp.com".to_string(),
                )
            } else if mfg_lower.contains("oneplus") {
                (
                    "OnePlus Official Account Recovery Guidance".to_string(),
                    "OxygenOS devices are protected by Google FRP. Universal Android Tool guides authorized recovery without security compromises.".to_string(),
                    vec![
                        "OnePlus Account or Google Account credentials".to_string(),
                        "Original purchase documentation".to_string(),
                    ],
                    vec![
                        "Step 1: Enter the Google Account credentials associated with the OnePlus device.".to_string(),
                        "Step 2: Reset credentials at https://accounts.google.com/signin/recovery or https://www.oneplus.com/support.".to_string(),
                        "Step 3: For hardware repair or recovery assistance, book service with OnePlus Care.".to_string(),
                    ],
                    "https://www.oneplus.com/support".to_string(),
                )
            } else if mfg_lower.contains("nokia") {
                (
                    "Nokia Android Official Ownership Verification & Recovery".to_string(),
                    "Nokia smartphones running Android One require standard Google Account verification. Zero bypass is permitted.".to_string(),
                    vec![
                        "Original proof of purchase".to_string(),
                        "Google Account credentials".to_string(),
                    ],
                    vec![
                        "Step 1: Enter the original Google Account credentials.".to_string(),
                        "Step 2: Complete Google Account Recovery at https://accounts.google.com/signin/recovery.".to_string(),
                        "Step 3: Contact HMD Global Phone Care at https://www.nokia.com/phones/support.".to_string(),
                    ],
                    "https://www.nokia.com/phones/support".to_string(),
                )
            } else {
                (
                    format!("{} Official Android FRP Guidance", device.manufacturer),
                    "Factory Reset Protection (FRP) is a standard Android security mechanism designed to prevent unauthorized access. Universal Android Tool does NOT circumvent FRP. Legitimate recovery requires original owner credentials or authorized service centers.".to_string(),
                    vec![
                        "Proof of purchase documentation".to_string(),
                        "Primary Google Account credentials".to_string(),
                    ],
                    vec![
                        "Step 1: Connect device to a trusted Wi-Fi network.".to_string(),
                        "Step 2: Enter the Google Account username and password associated with the device.".to_string(),
                        "Step 3: If credentials are forgotten, follow the official Google Account recovery flow at https://accounts.google.com/signin/recovery.".to_string(),
                        "Step 4: If credentials cannot be recovered, contact the manufacturer's authorized service center with proof of ownership.".to_string(),
                    ],
                    "https://support.google.com/android/answer/9459346".to_string(),
                )
            };

        RecoveryGuide {
            id: format!("frp-guide-{}", device.manufacturer.to_lowercase().replace(' ', "-")),
            manufacturer: device.manufacturer.clone(),
            title,
            category: "frp_recovery".to_string(),
            legal_notice,
            prerequisites,
            official_steps,
            official_support_url,
        }
    }

    /// Verifies if an authorized recovery action can be performed
    pub fn can_perform_authorized_reset(device: &DeviceInfo, verification: &OwnershipVerification) -> bool {
        verification.is_authorized_for_recovery() && device.capabilities.authorized_reset_supported
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samsung_tailored_guidance() {
        let device = DeviceInfo {
            id: "samsung-dev".to_string(),
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

        let guide = FrpRecoveryWorkflow::get_official_guidance(&device);
        assert!(guide.title.contains("Samsung"));
        assert!(guide.legal_notice.contains("Knox"));
        assert!(guide.official_support_url.contains("samsung.com"));
    }

    #[test]
    fn test_pixel_tailored_guidance() {
        let device = DeviceInfo {
            id: "pixel-dev".to_string(),
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

        let guide = FrpRecoveryWorkflow::get_official_guidance(&device);
        assert!(guide.title.contains("Pixel"));
        assert!(guide.official_support_url.contains("google.com"));
    }
}
