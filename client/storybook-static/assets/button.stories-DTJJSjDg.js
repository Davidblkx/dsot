import{c as r}from"./iframe--f-C4sbL.js";import"./preload-helper-Dp1pzeXC.js";const{action:s}=__STORYBOOK_MODULE_ACTIONS__,c={title:"Design System/Components/Button",argTypes:{value:{control:"text"},disabled:{control:"boolean"}}},e={render:t=>({render:()=>r("button",{type:"button",onClick:t.onClick,disabled:t.disabled},[t.value])}),args:{value:"Button",onClick:s("clicked"),disabled:!1}};var n,o,a;e.parameters={...e.parameters,docs:{...(n=e.parameters)==null?void 0:n.docs,source:{originalSource:`{
  render: args => ({
    render: () => <button type="button" onClick={args.onClick} disabled={args.disabled}>{args.value}</button>
  }),
  args: {
    value: 'Button',
    onClick: action('clicked'),
    disabled: false
  }
}`,...(a=(o=e.parameters)==null?void 0:o.docs)==null?void 0:a.source}}};const i=["Primary"];export{e as Primary,i as __namedExportsOrder,c as default};
