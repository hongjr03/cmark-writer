export default {
  title: "cmark-writer",

  locales: {
    root: {
      label: 'English',
      lang: 'en',
      description: 'A Rust library for serializing CommonMark AST nodes into CommonMark format',
    },
    'zh-CN': {
      label: '中文',
      lang: 'zh-CN',
      description: '一个 Rust 库，用于将 CommonMark AST 节点序列化为 CommonMark 格式',
    }
  },

  themeConfig: {
    logo: "/logo.png",

    localeLinks: {
      text: 'Language',
      items: [
        { text: 'English', link: '/' },
        { text: '中文', link: '/zh-CN/' },
      ]
    },

    // 英文导航
    nav: [
      { text: "Home", link: "/" },
      { text: "Guide", link: "/guide/introduction" },
      { text: "API Reference", link: "/api/index" },
      { text: "GitHub", link: "https://github.com/hongjr03/cmark-rs" },
    ],

    // 中文导航
    'zh-CN': {
      nav: [
        { text: "首页", link: "/zh-CN/" },
        { text: "指南", link: "/zh-CN/guide/introduction" },
        { text: "API 参考", link: "/zh-CN/api/index" },
        { text: "GitHub", link: "https://github.com/hongjr03/cmark-rs" },
      ],
    },

    // 英文侧边栏
    sidebar: {
      "/guide/": [
        {
          text: "Introduction",
          items: [
            { text: "Introduction", link: "/guide/introduction" },
            { text: "Getting Started", link: "/guide/getting-started" },
          ],
        },
        {
          text: "Core Concepts",
          items: [
            { text: "Overview", link: "/guide/core-concepts/index" },
            { text: "AST Nodes", link: "/guide/core-concepts/ast-nodes" },
            { text: "Writer Interface", link: "/guide/core-concepts/writer-interface" },
            { text: "Options", link: "/guide/core-concepts/options" },
          ],
        },
        {
          text: "Advanced Usage",
          items: [
            { text: "Overview", link: "/guide/advanced-usage/index" },
            { text: "Custom Nodes", link: "/guide/advanced-usage/custom-nodes" },
            { text: "Error Handling", link: "/guide/advanced-usage/error-handling" },
            { text: "GFM Extensions", link: "/guide/advanced-usage/gfm-extensions" },
          ],
        },
        {
          text: "Examples",
          items: [
            { text: "Overview", link: "/guide/examples/index" },
            { text: "Basic Document", link: "/guide/examples/basic-document" },
            { text: "Tables", link: "/guide/examples/tables" },
            { text: "Task Lists", link: "/guide/examples/task-lists" },
          ],
        },
        {
          text: "Contributing",
          items: [
            { text: "Contributing", link: "/guide/contributing" },
          ],
        },
      ],
      "/api/": [
        {
          text: "API Reference",
          items: [
            { text: "Overview", link: "/api/index" },
            { text: "Node", link: "/api/node" },
            { text: "CommonMarkWriter", link: "/api/writer" },
            { text: "WriterOptions", link: "/api/options" },
          ],
        },
      ],

      // 中文侧边栏
      "/zh-CN/guide/": [
        {
          text: "介绍",
          items: [
            { text: "介绍", link: "/zh-CN/guide/introduction" },
            { text: "快速开始", link: "/zh-CN/guide/getting-started" },
          ],
        },
        {
          text: "核心概念",
          items: [
            { text: "概述", link: "/zh-CN/guide/core-concepts/index" },
            { text: "AST 节点", link: "/zh-CN/guide/core-concepts/ast-nodes" },
            { text: "编写器接口", link: "/zh-CN/guide/core-concepts/writer-interface" },
            { text: "格式化选项", link: "/zh-CN/guide/core-concepts/options" },
          ],
        },
        {
          text: "高级用法",
          items: [
            { text: "概述", link: "/zh-CN/guide/advanced-usage/index" },
            { text: "自定义节点", link: "/zh-CN/guide/advanced-usage/custom-nodes" },
            { text: "错误处理", link: "/zh-CN/guide/advanced-usage/error-handling" },
            { text: "GFM 扩展", link: "/zh-CN/guide/advanced-usage/gfm-extensions" },
          ],
        },
        {
          text: "示例",
          items: [
            { text: "概述", link: "/zh-CN/guide/examples/index" },
            { text: "基础文档", link: "/zh-CN/guide/examples/basic-document" },
            { text: "表格", link: "/zh-CN/guide/examples/tables" },
            { text: "GFM 任务列表", link: "/zh-CN/guide/examples/task-lists" },
          ],
        },
        {
          text: "贡献指南",
          items: [
            { text: "贡献指南", link: "/zh-CN/guide/contributing" },
          ],
        },
      ],
      "/zh-CN/api/": [
        {
          text: "API 参考",
          items: [
            { text: "概述", link: "/zh-CN/api/index" },
            { text: "Node", link: "/zh-CN/api/node" },
            { text: "CommonMarkWriter", link: "/zh-CN/api/writer" },
            { text: "WriterOptions", link: "/zh-CN/api/options" },
          ],
        },
      ],
    },

    socialLinks: [
      { icon: "github", link: "https://github.com/hongjr03/cmark-rs" },
    ],
  },
  head: [
    ["link", { rel: "icon", type: "image/png", href: "/favicon.png" }],
  ],
}