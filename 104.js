"use strict";(self.webpackChunklinguistic_flux_capacitor=self.webpackChunklinguistic_flux_capacitor||[]).push([[104],{104:(e,t,r)=>{r.a(e,(async(e,n)=>{try{r.r(t),r.d(t,{loadFile:()=>o});var _=r(150),i=r(539),a=r(828),s=e([_,i]);async function o(){const e=_.j4.new();let t,r=await fetch(a),n=r.headers.get("content-length"),s=0,o=r.body.getReader(),l=document.getElementById("downloadProgressBar"),d=document.getElementById("downloadProgressText");for(;void 0===t;){let{value:r,done:n}=await o.read();if(n)throw"Exited before header was read";if(0!==r.length){let n=e.reserve(r.length);new Uint8Array(i.memory.buffer,n,s+r.length).set(r,s),s+=r.length,t=e.avail(r.length)}}if(n&&n!=t.len)throw"File size in HTTP header does not match file size in file header.";if(s>t.len)throw"File larger than expected.";let g=new Uint8Array(i.memory.buffer,t.pointer,t.len);for(;;){let e=100*s/t.len;l.style.width=e+"%",d.innerText=Math.floor(e)+" %";let{value:r,done:n}=await o.read();if(n)break;if(s+r.length>t.len)throw"File larger than expected.";0===g.length&&(g=new Uint8Array(i.memory.buffer,t.pointer,t.len)),g.set(r,s),s+=r.length}if(s!=t.len)throw"File smaller than expected.";return e.finish()}[_,i]=s.then?(await s)():s,_.p9(),n()}catch(l){n(l)}}))},828:(e,t,r)=>{e.exports=r.p+"30de1b633f2935887848.dwe"},150:(e,t,r)=>{r.a(e,(async(e,n)=>{try{r.d(t,{j4:()=>i.j4,p9:()=>i.p9});var _=r(539),i=r(282),a=e([_]);_=(a.then?(await a)():a)[0],(0,i.lI)(_),_.__wbindgen_start(),n()}catch(e){n(e)}}))},282:(e,t,r)=>{let n;function _(e){n=e}r.d(t,{Qn:()=>v,bL:()=>m,j4:()=>u,lI:()=>_,p9:()=>c});let i=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});i.decode();let a=null;let s=null;function o(){return null!==s&&0!==s.byteLength||(s=new Uint32Array(n.memory.buffer)),s}let l=0;function d(e,t){const r=t(4*e.length,4)>>>0;return o().set(e,r/4),l=e.length,r}let g=null;function w(e,t){return e>>>=0,o().subarray(e/4,e/4+t)}function c(){n.set_panic_hook()}const b="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>n.__wbg_embeddingfilebuilder_free(e>>>0,1)));class u{static __wrap(e){e>>>=0;const t=Object.create(u.prototype);return t.__wbg_ptr=e,b.register(t,t.__wbg_ptr,t),t}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,b.unregister(this),e}free(){const e=this.__destroy_into_raw();n.__wbg_embeddingfilebuilder_free(e,0)}static new(){const e=n.embeddingfilebuilder_new();return u.__wrap(e)}reserve(e){return n.embeddingfilebuilder_reserve(this.__wbg_ptr,e)>>>0}avail(e){const t=n.embeddingfilebuilder_avail(this.__wbg_ptr,e);return 0===t?void 0:y.__wrap(t)}finish(){const e=this.__destroy_into_raw(),t=n.embeddingfilebuilder_finish(e);return h.__wrap(t)}}const f="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>n.__wbg_embeddinghandle_free(e>>>0,1)));class h{static __wrap(e){e>>>=0;const t=Object.create(h.prototype);return t.__wbg_ptr=e,f.register(t,t.__wbg_ptr,t),t}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,f.unregister(this),e}free(){const e=this.__destroy_into_raw();n.__wbg_embeddinghandle_free(e,0)}pairwise_trajectories(e,t){const r=d(e,n.__wbindgen_malloc),_=l,i=d(t,n.__wbindgen_malloc),a=l,s=n.embeddinghandle_pairwise_trajectories(this.__wbg_ptr,r,_,i,a);var o,w,c=(o=s[0],w=s[1],o>>>=0,(null!==g&&0!==g.byteLength||(g=new Float32Array(n.memory.buffer)),g).subarray(o/4,o/4+w)).slice();return n.__wbindgen_free(s[0],4*s[1],4),c}most_related_to_at_t(e,t,r){const _=d(e,n.__wbindgen_malloc),i=l,a=n.embeddinghandle_most_related_to_at_t(this.__wbg_ptr,_,i,t,r);var s=w(a[0],a[1]).slice();return n.__wbindgen_free(a[0],4*a[1],4),s}largest_changes_wrt(e,t,r,_){const i=n.embeddinghandle_largest_changes_wrt(this.__wbg_ptr,e,t,r,_);var a=w(i[0],i[1]).slice();return n.__wbindgen_free(i[0],4*i[1],4),a}}const p="undefined"==typeof FinalizationRegistry?{register:()=>{},unregister:()=>{}}:new FinalizationRegistry((e=>n.__wbg_pointerandlen_free(e>>>0,1)));class y{static __wrap(e){e>>>=0;const t=Object.create(y.prototype);return t.__wbg_ptr=e,p.register(t,t.__wbg_ptr,t),t}__destroy_into_raw(){const e=this.__wbg_ptr;return this.__wbg_ptr=0,p.unregister(this),e}free(){const e=this.__destroy_into_raw();n.__wbg_pointerandlen_free(e,0)}get pointer(){return n.__wbg_get_pointerandlen_pointer(this.__wbg_ptr)>>>0}set pointer(e){n.__wbg_set_pointerandlen_pointer(this.__wbg_ptr,e)}get len(){return n.__wbg_get_pointerandlen_len(this.__wbg_ptr)>>>0}set len(e){n.__wbg_set_pointerandlen_len(this.__wbg_ptr,e)}}function m(){const e=n.__wbindgen_export_0,t=e.grow(4);e.set(0,void 0),e.set(t+0,void 0),e.set(t+1,null),e.set(t+2,!0),e.set(t+3,!1)}function v(e,t){throw new Error((r=e,_=t,r>>>=0,i.decode((null!==a&&0!==a.byteLength||(a=new Uint8Array(n.memory.buffer)),a).subarray(r,r+_))));var r,_}},539:(e,t,r)=>{var n=r(282);e.exports=r.v(t,e.id,"0c1781c88118c1afefbf",{"./linguistic_flux_capacitor_backend_bg.js":{__wbindgen_throw:n.Qn,__wbindgen_init_externref_table:n.bL}})}}]);