pub const XML_PATH: &str = "locale.xml";
pub const SCRIPT_PATH: &str = "locale.bat";
pub const REBOOT_REGISTRY_PATH: &str = "SOFTWARE\\AAAHelperCo\\AAAHelperApp";
pub const REGISTRY_STARTUP_PATH: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run";
pub const REGISTRY_RUNONCE_PATH: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\RunOnce";
pub const REGISTRY_RESTORE_EXECUTABLE: &str = "registry_restore.exe";
pub const DATA_FOLDER_NAME: &str = "data";
pub const SETUP_FILE_NAME: &str = "setup.bin";
pub const SETUP_EXE_NAME: &str = "setup.exe";
pub const XML_CONTENT: &str = r#"<?xml version="1.0"?>
    <gs:GlobalizationServices xmlns:gs="urn:longhornGlobalizationUnattend">
        <gs:UserList>
            <gs:User CopySettingsToSystemAcct="true" CopySettingsToDefaultUserAcct="true"
                UserID="Current" />
        </gs:UserList>

        <!-- MUI language preferences -->
        <gs:MUILanguagePreferences>
            <gs:MUILanguage Value="fa-IR" />
            <gs:MUIFallback Value="en-US" />
        </gs:MUILanguagePreferences>

        <!--User
        Locale-->
        <gs:UserLocale>
            <gs:Locale SetAsCurrent="true" Name="fa-IR" />
        </gs:UserLocale>

        <!-- Non Unicode programs -->
        <gs:SystemLocale Name="fa-IR" />

        <gs:InputPreferences>
            <gs:InputLanguageID Action="add" ID="0429:00000429" />
        </gs:InputPreferences>
    </gs:GlobalizationServices>"#;
