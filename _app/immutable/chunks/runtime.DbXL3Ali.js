var Fn=Array.isArray,$n=Array.from,Bn=Object.isFrozen,Cn=Object.defineProperty,Gn=Object.getOwnPropertyDescriptor,Hn=Object.getOwnPropertyDescriptors,In=Object.prototype,Jn=Array.prototype,rn=Object.getPrototypeOf;const Kn=()=>{};function Ln(n){return n()}function en(n){for(var t=0;t<n.length;t++)n[t]()}const x=2,fn=4,O=8,un=16,w=32,K=64,j=128,B=256,g=512,b=1024,S=2048,m=4096,P=8192,Nn=16384,L=32768,Qn=65536,on=1<<18,N=Symbol("$state"),Rn=Symbol("$state.frozen");function sn(n){return n===this.v}function an(n,t){return n!=n?t==t:n!==t||n!==null&&typeof n=="object"||typeof n=="function"}function Q(n){return!an(n,this.v)}function Un(n){throw new Error("effect_in_teardown")}function Vn(){throw new Error("effect_in_unowned_derived")}function Wn(n){throw new Error("effect_orphan")}function Xn(){throw new Error("effect_update_depth_exceeded")}function Yn(){throw new Error("hydration_failed")}function Zn(n){throw new Error("props_invalid_value")}function nt(){throw new Error("state_unsafe_mutation")}function R(n){return{f:0,v:n,reactions:null,equals:sn,version:0}}function tt(n){var l;const t=R(n);return t.equals=Q,v!==null&&v.l!==null&&((l=v.l).s??(l.s=[])).push(t),t}function lt(n,t){return s!==null&&Z()&&s.f&x&&nt(),n.equals(t)||(n.v=t,n.version=On(),cn(n,b),Z()&&i!==null&&i.f&g&&!(i.f&w)&&(c!==null&&c.includes(n)?(h(i,b),J(i)):_===null?gt([n]):_.push(n))),t}function cn(n,t){var l=n.reactions;if(l!==null)for(var r=Z(),f=l.length,e=0;e<f;e++){var u=l[e],o=u.f;o&b||!r&&u===i||(h(u,t),o&(g|j)&&(o&x?cn(u,S):J(u)))}}function vn(n){i===null&&s===null&&Wn(),s!==null&&s.f&j&&Vn(),Y&&Un()}function pn(n,t){var l=t.last;l===null?t.last=t.first=n:(l.next=n,n.prev=l,t.last=n)}function k(n,t,l,r=!0){var f=(n&K)!==0,e={ctx:v,deps:null,nodes:null,f:n|b,first:null,fn:t,last:null,next:null,parent:f?null:i,prev:null,teardown:null,transitions:null,version:0};if(l){var u=D;try{mn(!0),I(e),e.f|=Nn}catch(a){throw z(e),a}finally{mn(u)}}else t!==null&&J(e);var o=l&&e.deps===null&&e.first===null&&e.nodes===null&&e.teardown===null;return!o&&!f&&r&&(i!==null&&pn(e,i),s!==null&&s.f&x&&pn(e,s)),e}function rt(n){const t=k(O,null,!1);return h(t,g),t.teardown=n,t}function et(n){vn();var t=i!==null&&(i.f&O)!==0&&v!==null&&!v.m;if(t){var l=v;(l.e??(l.e=[])).push(n)}else{var r=U(n);return r}}function ft(n){return vn(),V(n)}function ut(n){const t=k(K,n,!0);return()=>{z(t)}}function U(n){return k(fn,n,!1)}function V(n){return k(O,n,!0)}function ot(n){return V(n)}function it(n,t=0){return k(O|un|t,n,!0)}function st(n,t=!0){return k(O|w,n,!0,t)}function dn(n){var t=n.teardown;if(t!==null){const l=Y,r=s;En(!0),qn(null);try{t.call(null)}finally{En(l),qn(r)}}}function z(n,t=!0){var l=!1;if((t||n.f&on)&&n.nodes!==null){for(var r=n.nodes.start,f=n.nodes.end;r!==null;){var e=r===f?null:r.nextSibling;r.remove(),r=e}l=!0}if(nn(n,t&&!l),H(n,0),h(n,P),n.transitions)for(const o of n.transitions)o.stop();dn(n);var u=n.parent;u!==null&&n.f&w&&u.first!==null&&hn(n),n.next=n.prev=n.teardown=n.ctx=n.deps=n.parent=n.fn=n.nodes=null}function hn(n){var t=n.parent,l=n.prev,r=n.next;l!==null&&(l.next=r),r!==null&&(r.prev=l),t!==null&&(t.first===n&&(t.first=r),t.last===n&&(t.last=l))}function at(n,t){var l=[];W(n,l,!0),yn(l,()=>{z(n),t&&t()})}function yn(n,t){var l=n.length;if(l>0){var r=()=>--l||t();for(var f of n)f.out(r)}else t()}function W(n,t,l){if(!(n.f&m)){if(n.f^=m,n.transitions!==null)for(const u of n.transitions)(u.is_global||l)&&t.push(u);for(var r=n.first;r!==null;){var f=r.next,e=(r.f&L)!==0||(r.f&w)!==0;W(r,t,e?l:!1),r=f}}}function ct(n){gn(n,!0)}function gn(n,t){if(n.f&m){n.f^=m,$(n)&&I(n);for(var l=n.first;l!==null;){var r=l.next,f=(l.f&L)!==0||(l.f&w)!==0;gn(l,f?t:!1),l=r}if(n.transitions!==null)for(const e of n.transitions)(e.is_global||t)&&e.in()}}let C=!1,X=[];function wn(){C=!1;const n=X.slice();X=[],en(n)}function vt(n){C||(C=!0,queueMicrotask(wn)),X.push(n)}function pt(){C&&wn()}function bn(n){let t=x|b;i===null&&(t|=j);const l={deps:null,deriveds:null,equals:sn,f:t,first:null,fn:n,last:null,reactions:null,v:null,version:0};if(s!==null&&s.f&x){var r=s;r.deriveds===null?r.deriveds=[l]:r.deriveds.push(l)}return l}function dt(n){const t=bn(n);return t.equals=Q,t}function _n(n){nn(n);var t=n.deriveds;if(t!==null){n.deriveds=null;for(var l=0;l<t.length;l+=1)ht(t[l])}}function xn(n){_n(n);var t=Sn(n),l=(M||n.f&j)&&n.deps!==null?S:g;h(n,l),n.equals(t)||(n.v=t,n.version=On())}function ht(n){_n(n),H(n,0),h(n,P),n.first=n.last=n.deps=n.reactions=n.fn=null}const jn=0,yt=1;let G=jn,F=!1,D=!1,Y=!1;function mn(n){D=n}function En(n){Y=n}let E=[],T=0,s=null;function qn(n){s=n}let i=null,c=null,p=0,_=null;function gt(n){_=n}let An=0,M=!1,v=null;function On(){return An++}function Z(){return v!==null&&v.l===null}function $(n){var u,o;var t=n.f;if(t&b)return!0;if(t&S){var l=n.deps;if(l!==null){var r=(t&j)!==0,f;if(t&B){for(f=0;f<l.length;f++)((u=l[f]).reactions??(u.reactions=[])).push(n);n.f^=B}for(f=0;f<l.length;f++){var e=l[f];if($(e)&&xn(e),e.version>n.version)return!0;r&&!M&&!((o=e==null?void 0:e.reactions)!=null&&o.includes(n))&&(e.reactions??(e.reactions=[])).push(n)}}h(n,g)}return!1}function wt(n,t,l){throw n}function Sn(n){var t=c,l=p,r=_,f=s,e=M;c=null,p=0,_=null,s=n.f&(w|K)?null:n,M=!D&&(n.f&j)!==0;try{var u=(0,n.fn)(),o=n.deps;if(c!==null){var a,d;if(o!==null){var q=p===0?c:o.slice(0,p).concat(c),A=q.length>16?new Set(q):null;for(d=p;d<o.length;d++)a=o[d],(A!==null?!A.has(a):!q.includes(a))&&Pn(n,a)}if(o!==null&&p>0)for(o.length=p+c.length,d=0;d<c.length;d++)o[p+d]=c[d];else n.deps=o=c;if(!M)for(d=p;d<o.length;d++){a=o[d];var y=a.reactions;y===null?a.reactions=[n]:y[y.length-1]!==n&&!y.includes(n)&&y.push(n)}}else o!==null&&p<o.length&&(H(n,p),o.length=p);return u}finally{c=t,p=l,_=r,s=f,M=e}}function Pn(n,t){const l=t.reactions;let r=0;if(l!==null){r=l.length-1;const f=l.indexOf(n);f!==-1&&(r===0?t.reactions=null:(l[f]=l[r],l.pop()))}r===0&&t.f&x&&(h(t,S),t.f&(j|B)||(t.f^=B),H(t,0))}function H(n,t){var l=n.deps;if(l!==null)for(var r=t===0?null:l.slice(0,t),f=new Set,e=t;e<l.length;e++){var u=l[e];f.has(u)||(f.add(u),(r===null||!r.includes(u))&&Pn(n,u))}}function nn(n,t=!1){var l=n.first;for(n.first=n.last=null;l!==null;){var r=l.next;z(l,t),l=r}}function I(n){var t=n.f;if(!(t&P)){h(n,g);var l=n.ctx,r=i,f=v;i=n,v=l;try{t&un||nn(n),dn(n);var e=Sn(n);n.teardown=typeof e=="function"?e:null,n.version=An}catch(u){wt(u)}finally{i=r,v=f}}}function kn(){T>1e3&&(T=0,Xn()),T++}function Dn(n){var t=n.length;if(t!==0){kn();var l=D;D=!0;try{for(var r=0;r<t;r++){var f=n[r];if(f.first===null&&!(f.f&w))Tn([f]);else{var e=[];Mn(f,e),Tn(e)}}}finally{D=l}}}function Tn(n){var t=n.length;if(t!==0)for(var l=0;l<t;l++){var r=n[l];!(r.f&(P|m))&&$(r)&&(I(r),r.deps===null&&r.first===null&&r.nodes===null&&(r.teardown===null?hn(r):r.fn=null))}}function bt(){if(F=!1,T>1001)return;const n=E;E=[],Dn(n),F||(T=0)}function J(n){G===jn&&(F||(F=!0,queueMicrotask(bt)));for(var t=n;t.parent!==null;){t=t.parent;var l=t.f;if(l&w){if(!(l&g))return;h(t,S)}}E.push(t)}function Mn(n,t){var l=n.first,r=[];n:for(;l!==null;){var f=l.f,e=(f&(P|m))===0,u=f&w,o=(f&g)!==0,a=l.first;if(e&&(!u||!o)){if(u&&h(l,g),f&O){if(!u&&$(l)&&(I(l),a=l.first),a!==null){l=a;continue}}else if(f&fn)if(u||o){if(a!==null){l=a;continue}}else r.push(l)}var d=l.next;if(d===null){let y=l.parent;for(;y!==null;){if(n===y)break n;var q=y.next;if(q!==null){l=q;continue n}y=y.parent}}l=d}for(var A=0;A<r.length;A++)a=r[A],t.push(a),Mn(a,t)}function tn(n,t=!0){var l=G,r=E;try{kn();const e=[];G=yt,E=e,F=!1,t&&Dn(r);var f=n==null?void 0:n();return pt(),(E.length>0||e.length>0)&&tn(),T=0,f}finally{G=l,E=r}}async function _t(){await Promise.resolve(),tn()}function zn(n){var t=n.f;if(t&P)return n.v;if(s!==null){var l=s.deps;c===null&&l!==null&&l[p]===n?p++:(l===null||p===0||l[p-1]!==n)&&(c===null?c=[n]:c[c.length-1]!==n&&c.push(n)),_!==null&&i!==null&&i.f&g&&!(i.f&w)&&_.includes(n)&&(h(i,b),J(i))}if(t&x){var r=n;$(r)&&xn(r)}return n.v}function xt(n){const t=s;try{return s=null,n()}finally{s=t}}const jt=~(b|S|g);function h(n,t){n.f=n.f&jt|t}function mt(n){return typeof n=="object"&&n!==null&&typeof n.f=="number"}function Et(n,t=!1,l){v={p:v,c:null,e:null,m:!1,s:n,x:null,l:null},t||(v.l={s:null,u:null,r1:[],r2:R(!1)})}function qt(n){const t=v;if(t!==null){const r=t.e;if(r!==null){t.e=null;for(var l=0;l<r.length;l++)U(r[l])}v=t.p,t.m=!0}return{}}function At(n){if(!(typeof n!="object"||!n||n instanceof EventTarget)){if(N in n)ln(n);else if(!Array.isArray(n))for(let t in n){const l=n[t];typeof l=="object"&&l&&N in l&&ln(l)}}}function ln(n,t=new Set){if(typeof n=="object"&&n!==null&&!(n instanceof EventTarget)&&!t.has(n)){t.add(n),n instanceof Date&&n.getTime();for(let r in n)try{ln(n[r],t)}catch{}const l=rn(n);if(l!==Object.prototype&&l!==Array.prototype&&l!==Map.prototype&&l!==Set.prototype&&l!==Date.prototype){const r=Hn(l);for(let f in r){const e=r[f].get;if(e)try{e.call(n)}catch{}}}}}function Ot(n){return mt(n)?zn(n):n}export{R as A,W as B,yn as C,z as D,Ot as E,Bn as F,N as G,on as H,m as I,In as J,Jn as K,Gn as L,rn as M,L as N,U as O,V as P,vt as Q,Zn as R,Rn as S,Qn as T,Q as U,bn as V,dt as W,_t as X,an as Y,$n as a,st as b,i as c,Cn as d,ut as e,tn as f,qt as g,Yn as h,Fn as i,v as j,et as k,xt as l,zn as m,At as n,Ln as o,Et as p,Kn as q,en as r,tt as s,rt as t,ft as u,lt as v,ot as w,it as x,ct as y,at as z};
