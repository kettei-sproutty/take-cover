import{e as O,d as P,f as b,b as y,s as S,a as V,c as p,g as $,i as q,j as M,k as B,l as D,h as k,m as N,P as W}from"./disclose-version.BDO1LOx9.js";import{d as z,i as A,h as F,c as G,e as H,a as I,f as J,g as R,j as U,k as X}from"./runtime.CMm55TBp.js";const T=new Set,x=new Set;function Y(t){for(var a=0;a<t.length;a++)T.add(t[a]);for(var o of x)o(t)}function _(t){var L;var a=this,o=a.ownerDocument,l=t.type,i=((L=t.composedPath)==null?void 0:L.call(t))||[],e=i[0]||t.target,d=0,u=t.__root;if(u){var c=i.indexOf(u);if(c!==-1&&(a===document||a===window)){t.__root=a;return}var f=i.indexOf(a);if(f===-1)return;c<=f&&(d=c)}if(e=i[d]||t.target,e!==a){z(t,"currentTarget",{configurable:!0,get(){return e||o}});try{for(var v,r=[];e!==null;){var s=e.parentNode||e.host||null;try{var n=e["__"+l];if(n!==void 0&&!e.disabled)if(A(n)){var[g,...m]=n;g.apply(e,[t,...m])}else n.call(e,t)}catch(w){v?r.push(w):v=w}if(t.cancelBubble||s===a||s===null)break;e=s}if(v){for(let w of r)queueMicrotask(()=>{throw w});throw v}}finally{t.__root=a,e=a}}}function Z(t,a){(t.__t??(t.__t=t.nodeValue))!==a&&(t.nodeValue=t.__t=a)}function j(t,a){const o=a.anchor??a.target.appendChild(O());return C(t,{...a,anchor:o})}function K(t,a){a.intro=a.intro??!1;const o=a.target,l=k,i=p;try{for(var e=o.firstChild;e&&(e.nodeType!==8||e.data!==P);)e=e.nextSibling;if(!e)throw b;y(!0),S(e),V();const d=C(t,{...a,anchor:e});if(p===null||p.nodeType!==8||p.data!==$)throw q(),b;return y(!1),d}catch(d){if(d===b)return a.recover===!1&&F(),M(),B(o),y(!1),j(t,a);throw d}finally{y(l),S(i),D()}}const h=new Map;function C(t,{target:a,anchor:o,props:l={},events:i,context:e,intro:d=!0}){M();var u=new Set,c=r=>{for(var s=0;s<r.length;s++){var n=r[s];if(!u.has(n)){u.add(n);var g=W.includes(n);a.addEventListener(n,_,{passive:g});var m=h.get(n);m===void 0?(document.addEventListener(n,_,{passive:g}),h.set(n,1)):h.set(n,m+1)}}};c(G(T)),x.add(c);var f=void 0,v=H(()=>(I(()=>{if(e){J({});var r=X;r.c=e}i&&(l.$$events=i),k&&N(o,null),f=t(o,l)||{},k&&(R.nodes.end=p),e&&U()}),()=>{for(var r of u){a.removeEventListener(r,_);var s=h.get(r);--s===0?(document.removeEventListener(r,_),h.delete(r)):h.set(r,s)}x.delete(c),E.delete(f)}));return E.set(f,v),f}let E=new WeakMap;function Q(t){var a;(a=E.get(t))==null||a()}export{Y as d,K as h,j as m,Z as s,Q as u};