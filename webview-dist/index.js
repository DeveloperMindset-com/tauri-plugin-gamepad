function n(n,e,t,r){return new(t||(t=Promise))((function(a,i){function u(n){try{c(r.next(n))}catch(n){i(n)}}function o(n){try{c(r.throw(n))}catch(n){i(n)}}function c(n){var e;n.done?a(n.value):(e=n.value,e instanceof t?e:new t((function(n){n(e)}))).then(u,o)}c((r=r.apply(n,e||[])).next())}))}function e(n,e){var t,r,a,i,u={label:0,sent:function(){if(1&a[0])throw a[1];return a[1]},trys:[],ops:[]};return i={next:o(0),throw:o(1),return:o(2)},"function"==typeof Symbol&&(i[Symbol.iterator]=function(){return this}),i;function o(o){return function(c){return function(o){if(t)throw new TypeError("Generator is already executing.");for(;i&&(i=0,o[0]&&(u=0)),u;)try{if(t=1,r&&(a=2&o[0]?r.return:o[0]?r.throw||((a=r.return)&&a.call(r),0):r.next)&&!(a=a.call(r,o[1])).done)return a;switch(r=0,a&&(o=[2&o[0],a.value]),o[0]){case 0:case 1:a=o;break;case 4:return u.label++,{value:o[1],done:!1};case 5:u.label++,r=o[1],o=[0];continue;case 7:o=u.ops.pop(),u.trys.pop();continue;default:if(!(a=u.trys,(a=a.length>0&&a[a.length-1])||6!==o[0]&&2!==o[0])){u=0;continue}if(3===o[0]&&(!a||o[1]>a[0]&&o[1]<a[3])){u.label=o[1];break}if(6===o[0]&&u.label<a[1]){u.label=a[1],a=o;break}if(a&&u.label<a[2]){u.label=a[2],u.ops.push(o);break}a[2]&&u.ops.pop(),u.trys.pop();continue}o=e.call(n,u)}catch(n){o=[6,n],r=0}finally{t=a=0}if(5&o[0])throw o[1];return{value:o[0]?o[1]:void 0,done:!0}}([o,c])}}}function t(n,e=!1){return window.__TAURI_INTERNALS__.transformCallback(n,e)}async function r(n,e={},t){return window.__TAURI_INTERNALS__.invoke(n,e,t)}var a;async function i(n,e,a){return r("plugin:event|listen",{event:n,windowLabel:a?.target,handler:t(e)}).then((e=>async()=>async function(n,e){await r("plugin:event|unlisten",{event:n,eventId:e})}(n,e)))}"function"==typeof SuppressedError&&SuppressedError,"function"==typeof SuppressedError&&SuppressedError,function(n){n.WINDOW_RESIZED="tauri://resize",n.WINDOW_MOVED="tauri://move",n.WINDOW_CLOSE_REQUESTED="tauri://close-requested",n.WINDOW_CREATED="tauri://window-created",n.WINDOW_DESTROYED="tauri://destroyed",n.WINDOW_FOCUS="tauri://focus",n.WINDOW_BLUR="tauri://blur",n.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",n.WINDOW_THEME_CHANGED="tauri://theme-changed",n.WINDOW_FILE_DROP="tauri://file-drop",n.WINDOW_FILE_DROP_HOVER="tauri://file-drop-hover",n.WINDOW_FILE_DROP_CANCELLED="tauri://file-drop-cancelled"}(a||(a={}));var u,o=null,c=[],l=function(){return 0==c.length?[null,null,null,null]:c};r("plugin:gamepad|execute"),navigator.getGamepads=l,u=i("event",(function(n){var e=n.payload,t=d(e),r=!1;if(c=c.map((function(n){return n.id===t.id&&n.index===t.index?(r=!0,t):n})),r||c.push(t),"Connected"===e.event){var a=new Event("gamepadconnected");a.gamepad=t,window.dispatchEvent(a)}if("Disconnected"===e.event){var i=new Event("gamepaddisconnected");i.gamepad=t,window.dispatchEvent(i),c=c.filter((function(n){return n.index!==t.index}))}null!==o&&o(e)}));var d=function(n){var e=n.id,t=n.axes,r=n.connected,a=n.name,i=n.timestamp,u=n.uuid,o=n.mapping,c=n.buttons.map((function(n){return{value:n,touched:!1,pressed:n>0}}));return{index:e,id:"".concat(a," (").concat(u,")"),connected:r,axes:t,buttons:c,timestamp:i,mapping:o,hapticActuators:[],vibrationActuator:null}},s=function(t){return n(void 0,void 0,void 0,(function(){return e(this,(function(n){return o=t,[2,u]}))}))};export{s as execute,l as getGamepads};
