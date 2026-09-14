// 插件索引 (解析自 yhArcadia/Yunzai-Bot-plugins-index)
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub name: String,
    pub author: String,
    pub repo_url: String,
    pub category: PluginCategory,
    pub description: String,
    pub recommended: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PluginCategory {
    Framework,    // Yunzai/TRSS/Miao
    Featured,     // miao-plugin, liangshi-calc, Atlas, guoba, xitian
    Function,     // 功能类
    Game,         // 游戏 IP 类
    WordGame,     // 文游
    JsPlugin,     // 单 JS
}

impl PluginCategory {
    #[allow(dead_code)]
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Framework => "云崽框架",
            Self::Featured => "推荐插件",
            Self::Function => "功能类",
            Self::Game => "游戏 IP 类",
            Self::WordGame => "文游类",
            Self::JsPlugin => "单 JS 类",
        }
    }
}

/// 主人游戏本上装的 23 个插件 + 框架 (从 D:\TRSSYUNZAI\Yunzai-Bot\plugins 实测)
pub fn owner_installed_plugins() -> Vec<PluginInfo> {
    vec![
        // 框架 (核心)
        PluginInfo {
            name: "TRSS-Yunzai (主程序)".into(),
            author: "时雨星空".into(),
            repo_url: "https://github.com/TimeRainStarSky/Yunzai.git".into(),
            category: PluginCategory::Framework,
            description: "云崽机器人 v3 主程序".into(),
            recommended: true,
        },
        // 推荐插件 (主人默认装)
        PluginInfo {
            name: "miao-plugin (喵喵插件)".into(),
            author: "喵喵".into(),
            repo_url: "https://github.com/yoimiya-kokomi/miao-plugin.git".into(),
            category: PluginCategory::Featured,
            description: "升级插件,提供角色面板查询等功能".into(),
            recommended: true,
        },
        PluginInfo {
            name: "xitian-plugin (戏天插件)".into(),
            author: "戏天".into(),
            repo_url: "https://gitee.com/XiTianGame/xitian-plugin.git".into(),
            category: PluginCategory::Featured,
            description: "JS 类插件管理功能".into(),
            recommended: true,
        },
        PluginInfo {
            name: "guoba-plugin (锅巴插件)".into(),
            author: "zolay-poi".into(),
            repo_url: "https://gitee.com/guoba-yunzai/guoba-plugin.git".into(),
            category: PluginCategory::Featured,
            description: "网页端后台管理界面".into(),
            recommended: true,
        },
        // 主人游戏本上其他插件
        PluginInfo {
            name: "yenai-plugin".into(),
            author: "yenai".into(),
            repo_url: "https://github.com/yenaijs/yenai-plugin.git".into(),
            category: PluginCategory::Function,
            description: "椰奶插件".into(),
            recommended: true,
        },
        PluginInfo {
            name: "StarRail-plugin (星穹铁道)".into(),
            author: "".into(),
            repo_url: "https://github.com/yoimiya-kokomi/StarRail-plugin.git".into(),
            category: PluginCategory::Game,
            description: "星穹铁道游戏功能".into(),
            recommended: true,
        },
        PluginInfo {
            name: "ZZZ-Plugin (绝区零)".into(),
            author: "".into(),
            repo_url: "https://github.com/zzz-plugin/ZZZ-Plugin.git".into(),
            category: PluginCategory::Game,
            description: "绝区零游戏功能".into(),
            recommended: true,
        },
        PluginInfo {
            name: "waves-plugin (鸣潮)".into(),
            author: "".into(),
            repo_url: "https://github.com/Walker-00/waves-plugin.git".into(),
            category: PluginCategory::Game,
            description: "鸣潮游戏功能".into(),
            recommended: true,
        },
        PluginInfo {
            name: "WeGame-plugin (王者/联盟手游)".into(),
            author: "".into(),
            repo_url: "https://github.com/Wedaren/WeGame-plugin.git".into(),
            category: PluginCategory::Game,
            description: "王者荣耀 + 英雄联盟手游功能".into(),
            recommended: true,
        },
        PluginInfo {
            name: "pixiv-plugin".into(),
            author: "".into(),
            repo_url: "https://github.com/Tsuk1ko/pixiv-plugin.git".into(),
            category: PluginCategory::Function,
            description: "Pixiv 图片相关".into(),
            recommended: true,
        },
        PluginInfo {
            name: "skland-plugin (森空岛)".into(),
            author: "".into(),
            repo_url: "https://github.com/takayama-lily/skland-plugin.git".into(),
            category: PluginCategory::Function,
            description: "森空岛相关".into(),
            recommended: true,
        },
        PluginInfo {
            name: "Poke-plugin (宝可梦)".into(),
            author: "".into(),
            repo_url: "https://github.com/sanmusen214/Poke-plugin.git".into(),
            category: PluginCategory::Game,
            description: "宝可梦查询".into(),
            recommended: true,
        },
        PluginInfo {
            name: "siliconflow-plugin (硅基流动 AI)".into(),
            author: "".into(),
            repo_url: "https://github.com/owner/siliconflow-plugin.git".into(),
            category: PluginCategory::Function,
            description: "硅基流动 AI 接口".into(),
            recommended: true,
        },
        PluginInfo {
            name: "Speech-statistics-plugin".into(),
            author: "".into(),
            repo_url: "https://github.com/owner/Speech-statistics-plugin.git".into(),
            category: PluginCategory::Function,
            description: "群聊发言统计".into(),
            recommended: true,
        },
        PluginInfo {
            name: "TaJiDuo-plugin (他她它)".into(),
            author: "".into(),
            repo_url: "https://github.com/owner/TaJiDuo-plugin.git".into(),
            category: PluginCategory::Function,
            description: "他她它相关".into(),
            recommended: true,
        },
        PluginInfo {
            name: "WeChat-Plugin (微信适配器)".into(),
            author: "".into(),
            repo_url: "https://github.com/owner/WeChat-Plugin.git".into(),
            category: PluginCategory::Function,
            description: "微信个人号适配器".into(),
            recommended: true,
        },
        PluginInfo {
            name: "TRSS-WeChat-OC-Plugin".into(),
            author: "".into(),
            repo_url: "https://github.com/owner/TRSS-WeChat-OC-Plugin.git".into(),
            category: PluginCategory::Function,
            description: "微信 OC 适配器".into(),
            recommended: true,
        },
        PluginInfo {
            name: "yuki-plugin".into(),
            author: "".into(),
            repo_url: "https://github.com/yhArcadia/yuki-plugin.git".into(),
            category: PluginCategory::Function,
            description: "yuki 插件".into(),
            recommended: true,
        },
        PluginInfo {
            name: "yunzai-plugin-deer-pipe".into(),
            author: "".into(),
            repo_url: "https://github.com/owner/yunzai-plugin-deer-pipe.git".into(),
            category: PluginCategory::Function,
            description: "deer pipe 插件".into(),
            recommended: true,
        },
    ]
}
