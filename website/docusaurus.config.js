module.exports={
  title: "Catlang",
  tagline: "A strongly-typed language without the pain",
  url: "https://aszecsei.github.io",
  baseUrl: "/catlang/",
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: "img/favicon.png",
  organizationName: "aszecsei",
  projectName: "catlang",
  themeConfig: {
    sidebarCollapsible: false,
    navbar: {
      title: 'Catlang',
      logo: {
        alt: 'Catlang Logo',
        src: 'img/docusaurus.svg',
      },
      items: [
        {
          to: 'docs/',
          activeBasePath: 'docs',
          label: 'Docs',
          position: 'left',
        },
        {to: 'blog', label: 'Blog', position: 'left'},
        {
          href: 'https://github.com/aszecsei/catlang',
          label: 'GitHub',
          position: 'right',
        },
      ],
    },
    footer: {
      style: 'dark',
      links: [
        {
          title: 'Docs',
          items: [
            {
              label: 'Intro',
              to: 'docs/',
            },
            {
              label: 'Setup',
              to: 'docs/installation',
            },
            {
              label: 'Language Basics',
              to: 'docs/overview',
            },
            {
              label: 'API',
              to: 'docs/api/',
            },
          ],
        },
        {
          title: 'Community',
          items: [
            {
              label: 'Stack Overflow',
              href: 'https://stackoverflow.com/questions/tagged/catlang',
            },
            {
              label: 'Twitter',
              href: 'https://twitter.com/aszecsei',
            },
          ],
        },
        {
          title: 'More',
          items: [
            {
              label: 'Blog',
              to: 'blog',
            },
            {
              label: 'GitHub',
              href: 'https://github.com/aszecsei/catlang',
            },
          ],
        },
      ],
      copyright: `Copyright © ${new Date().getFullYear()} Alic Szecsei. Built with Docusaurus.`,
    },
  },
  "presets": [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/aszecsei/catlang/edit/master/website/',
        },
        blog: {
          showReadingTime: true,
          editUrl: 'https://github.com/aszecsei/catlang/edit/master/website/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css')
        }
      }
    ]
  ]
}