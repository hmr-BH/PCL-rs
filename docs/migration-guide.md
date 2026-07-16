# PCL-rs 迁移规范（PCL-CE → Rust + Slint）

本文件是 PCL-CE（WPF/C#）UI 向 PCL-rs（Rust + Slint 1.16）1:1 迁移的统一规范。
**所有 UI 代码必须遵守本规范。** 目标：外观、动画、交互、操作流程与原版一致。

- 原版项目：`D:\Projects\PCL-CE\Plain Craft Launcher 2\`（XAML + C#）
- Slint 官方文档（离线 HTML）：`D:\hmrBH\Slint Docs\`（不确定组件/属性用法时**必须先查文档再写代码**，重点看 `reference/` 下的 window、elements、layouts 与 `guide/`）
- 编译验证：`export PATH="$HOME/.cargo/bin:$PATH" && cd D:/Projects/PCL-rs && cargo check`
  （注意 bash 中 cargo 不在默认 PATH）

## 1. 目录结构与文件归属

```
ui/
  theme/theme.slint        GlobalTheme：完整色板（color1..8、gray、red、log 色等）+ 字体/圆角常量
  api/                     Rust↔Slint 接口契约（每域一个文件，见 §5）
  components/              基础组件（对应原版 Controls/My*.xaml(.cs)，以及纯 C# 控件如 MyTextBox）
  widgets/                 小部件（由 components 组合、面向具体业务，对应原版 Pages 内的 My*Item 等）
  pages/                   页面（对应原版 Pages/**.xaml，按原目录分子目录）
  assets/icons/lucide/     87 个 lucide SVG（从 PCL.Core 复制，stroke=currentColor）
  mainwindow.slint         AppWindow：标题栏 + 导航 + 左右页面区 + 对话框/Toast 浮层
src/
  main.rs                  入口：创建 AppWindow、装配 interface 模块
  interface/               后端空壳封装层（见 §5），按域分文件
docs/migration-guide.md    本文件
```

命名：组件沿用原命名（`MyButton`、`MyIconButton`…）；文件名小写（`icon_button.slint`）。
每个组件文件头部必须带注释：对应原版文件、关键动画参数、与原版的已知偏差（如有）。

## 2. 主题与字体

- 一律使用 `GlobalTheme` 中的颜色，**禁止散落硬编码**（页面特有的静态文本色可用 `GlobalTheme.color1/gray*`）。
- 字体：`GlobalTheme.font-family`（"Microsoft YaHei UI"），正文 `GlobalTheme.font-size`（13px），卡片标题 `GlobalTheme.title-font-weight`（700）。
- 标准圆角：按钮/输入框 3~4px、卡片 5px、对话框 7px、Toast 8px、导航 pill 13.5px、图标按钮圆形。
- 标准高度：输入框 28px、列表项 42px、导航 pill 27px、标题栏 48px、标题按钮 28×28、搜索框 40px、启动按钮 54px、卡片折叠高 40px。

## 3. 图标

原版用 lucide SVG（`SvgIcon`）+ 少量 WPF path（`Logo`）。在 Slint 中：

```slint
Image {
    width: 16px; height: 16px;
    source: @image-url("../assets/icons/lucide/x.svg");
    colorize: <brush>;        // 等价于原版 IconBrush 重着色
    image-fit: contain;
}
```

原版内联 path 图标（如卡片 chevron）用 Slint `Path { commands: "..." }` 实现（参考已有 `components/checkbox.slint` 的勾）。

## 4. 动画规范（最重要）

原版缓动（ModAnimation.cs）：`AniEaseOutFluent(p)=1-(1-t)^p`（Weak=2/Middle=3/Strong=4/XStrong=5）、
`AniEaseOutBack(p)=1-(1-t)^(3-p/2)·cos(1.5πt)`、`AniEaseInFluent(p)=t^p`。
Slint 中按已确立的近似映射（见 components/checkbox.slint、radiobox.slint、card.slint）：

| 原版 | Slint 写法 |
|---|---|
| OutFluent（悬停变色 90~200ms） | `animate color/background { duration: Nms; }`（默认缓动即可） |
| OutBack（弹性缩放/位移） | `animate x { duration: Nms; easing: cubic-bezier(0.28, 1.25, 0.50, 1.0); }` |
| 按下缩放（scale→0.955/0.98） | `transform-scale: touch.pressed ? 0.955 : 1.0; animate transform-scale { duration: 100ms; easing: ease-out; }` |
| 多段回弹序列（如复选框 18→10→18） | 用 `Timer` 链 + 状态属性分阶段驱动（见 checkbox.slint 的 `_bt/_bs` 模式） |
| 折叠高度动画 | `animate height { duration: 200ms; easing: cubic-bezier(0, 0.45, 0.05, 1); }` |

页面进入/退出动画（右页 MyPageRight）：各卡片/控件 `opacity 0→1 100ms` + `TranslateY -16→0`（+5/250ms OutFluent，+11/350ms OutBack），按控件顺序 stagger `delay+=25ms`；退出 `opacity→0 70ms` + `Y-6`，stagger 15ms。左页（MyPageLeft）：整体 `scale 0.96→1 OutBack(Weak)` 或子项 `TranslateX -25→0` stagger。Slint 中用逐元素 `animate` + Timer stagger 或状态驱动实现，视觉上保持一致即可。

## 5. Rust ↔ Slint 接口层（企业级封装）

**铁律：假数据与延时只允许出现在 `src/interface/*.rs`，绝不允许写进 .slint。**

- `ui/api/<域>.slint`：定义该域的 `export struct` 与 `export global XxxApi`（`in-out property` 放数据模型，`callback` 放前端→后端调用）。每域一个文件，避免冲突。
- `src/interface/<域>.rs`：实现对应 global 的 callback。空壳要求：
  1. 函数注释写清**未来真实实现要做什么**（参考原版 C# 逻辑）；
  2. 用假数据 + `slint::Timer`（或线程 + `slint::invoke_from_event_loop`）模拟异步延时，把结果写回 Slint 属性（`VecModel` / `ModelRc`）；
  3. 前端表现必须与原版一致（如加载动画转一会再出列表）。
- 域划分建议：`router`（导航/页面栈）、`launch`（启动/账户/主页）、`download`（版本与资源下载）、`instance`（实例管理）、`setup`（设置）、`tools`、`log`、`speed`（任务管理）、`msg`（对话框/Toast 队列）。
- main.rs 只做装配：`interface::mod::setup(&ui)` 一类调用，业务逻辑全部进 interface。

## 6. 页面规范

- 目录镜像原版：`ui/pages/launch/`、`pages/download/`、`pages/instance/`、`pages/setup/`、`pages/select/`、`pages/log/`、`pages/speed/`、`pages/tools/`。
- 每个 Right 页面根组件对应原版 MyPageRight：内含 `Flickable`（滚动）+ 卡片列表；卡片间距 15px、页面左右 padding 20px 级别（以原版 XAML 为准）。
- Left 页面（导航栏）对应 MyPageLeft：分类标题（12px、opacity 0.6、margin 13,5,5,3）+ `MyListItem type=RadioBox height=36` 带 lucide 图标。
- 页面需要后端数据时**只通过 `ui/api/` 的 global** 取数/回调，禁止在 .slint 里写假数据。
- 与原版一致的细节以原版 XAML 为准：padding、margin、字号、文案（中文文案照抄原版）。

## 7. 主窗体（mainwindow.slint）

- `AppWindow`：850×500（min 810×470），背景 `window-background`；`no-frame: true` 自定义标题栏（48px，`color3` 背景，白色 28×28 图标按钮：min/close；左侧 logo + 4 个白色 `MyRadioButton` 导航 pill：启动/下载/设置/工具）。
- 窗口拖拽：标题栏 TouchArea → 回调到 Rust，由 Rust 调 `window().set_position()` 实现（用按下时全局坐标差计算）。
- 左侧面板（`left-panel-background` 半透明）+ 右侧内容区；对话框浮层（PanMsg 等价物，半透明黑 `#5A000000` 遮罩 + 卡片旋转/位移进入动画）与 Toast 浮层（右下角，最多 5 条）挂在根节点。
- 页面切换动画按 §4 的页面规范实现；页面栈/后退逻辑放 slint 侧状态 + `router` 接口。

## 8. 提交流程

每阶段完成后：全量 `cargo check` 通过 → `git add -A && git commit`（中文提交信息，说明本阶段内容）。
**禁止**提交编译不通过的代码。
