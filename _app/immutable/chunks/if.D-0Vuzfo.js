import{w as j,x as q,y as x,o as h,n as E,a as T,b as p,p as z,z as A,A as k,v as G,B as H,h as w,C as I,D as N,H as P,m as R}from"./disclose-version.Y3a8vduN.js";import{d as W,e as F,R as J,T as K,U as Q,m as C,E as U,j as X,G as Y,C as Z,l as tt,M as B,p as D,I as et}from"./runtime.C9lo_CK4.js";const O=new Set,L=new Set;function at(t){for(var e=0;e<t.length;e++)O.add(t[e]);for(var s of L)s(t)}function b(t){var S;var e=this,s=e.ownerDocument,c=t.type,i=((S=t.composedPath)==null?void 0:S.call(t))||[],a=i[0]||t.target,n=0,r=t.__root;if(r){var f=i.indexOf(r);if(f!==-1&&(e===document||e===window)){t.__root=e;return}var u=i.indexOf(e);if(u===-1)return;f<=u&&(n=f)}if(a=i[n]||t.target,a!==e){W(t,"currentTarget",{configurable:!0,get(){return a||s}});try{for(var v,o=[];a!==null;){var d=a.parentNode||a.host||null;try{var l=a["__"+c];if(l!==void 0&&!a.disabled)if(F(l)){var[m,...y]=l;m.apply(a,[t,...y])}else l.call(a,t)}catch(_){v?o.push(_):v=_}if(t.cancelBubble||d===e||d===null)break;a=d}if(v){for(let _ of o)queueMicrotask(()=>{throw _});throw v}}finally{t.__root=e,a=e}}}function nt(t,e){(t.__t??(t.__t=t.nodeValue))!==e&&(t.nodeValue=t.__t=e)}function V(t,e){const s=e.anchor??e.target.appendChild(j());return $(t,{...e,anchor:s})}function ot(t,e){e.intro=e.intro??!1;const s=e.target,c=w,i=p;try{for(var a=s.firstChild;a&&(a.nodeType!==8||a.data!==q);)a=a.nextSibling;if(!a)throw x;h(!0),E(a),T();const n=$(t,{...e,anchor:a});if(p===null||p.nodeType!==8||p.data!==z)throw A(),x;return h(!1),n}catch(n){if(n===x)return e.recover===!1&&J(),k(),G(s),h(!1),V(t,e);throw n}finally{h(c),E(i),H()}}const g=new Map;function $(t,{target:e,anchor:s,props:c={},events:i,context:a,intro:n=!0}){k();var r=new Set,f=o=>{for(var d=0;d<o.length;d++){var l=o[d];if(!r.has(l)){r.add(l);var m=N.includes(l);e.addEventListener(l,b,{passive:m});var y=g.get(l);y===void 0?(document.addEventListener(l,b,{passive:m}),g.set(l,1)):g.set(l,y+1)}}};f(K(O)),L.add(f);var u=void 0,v=Q(()=>(C(()=>{if(a){U({});var o=Z;o.c=a}i&&(c.$$events=i),w&&I(s,null),u=t(s,c)||{},w&&(X.nodes.end=p),a&&Y()}),()=>{for(var o of r){e.removeEventListener(o,b);var d=g.get(o);--d===0?(document.removeEventListener(o,b),g.delete(o)):g.set(o,d)}L.delete(f),M.delete(u)}));return M.set(u,v),u}let M=new WeakMap;function st(t){var e;(e=M.get(t))==null||e()}function rt(t,e,s,c=null,i=!1){w&&T();var a=t,n=null,r=null,f=null,u=i?et:0;tt(()=>{if(f===(f=!!e()))return;let v=!1;if(w){const o=a.data===P;f===o&&(a=R(),E(a),h(!1),v=!0)}f?(n?B(n):n=C(()=>s(a)),r&&D(r,()=>{r=null})):(r?B(r):c&&(r=C(()=>c(a))),n&&D(n,()=>{n=null})),v&&h(!0)},u),w&&(a=p)}export{at as d,ot as h,rt as i,V as m,nt as s,st as u};