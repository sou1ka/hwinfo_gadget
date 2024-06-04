const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;

async function get_hwinfo() {
  let ret = await invoke("get_hwifo");
  let res = JSON.parse(ret);console.log(res);
  set_hwinfo(res);
}

async function set_hwinfo(res) {
  if(res) {
    document.querySelector('#container dl').remove();
    document.querySelector('#container').insertAdjacentHTML('afterbegin', '<dl></dl>');
    let t = document.querySelector('#container dl');
    let html = '';
    for(let m of res) {
      html += '<dt>' + m.Label + '</dt><dd>' + m.Value + '</dd>';
    }

    t.insertAdjacentHTML('afterbegin', html);
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  get_hwinfo();

  listen('hwinfo_refresh', function(ret) {
    let res = JSON.parse(ret.payload);
    set_hwinfo(res);
  });
});

document.addEventListener('keydown', async function(e) {
  if(e.key == 'F5' || (e.ctrlKey && e.key == 'r') || e.key == 'F7') {
    e.preventDefault();
    e.stopPropagation();

  } else if(e.altKey && e.key == 'Enter') {
    appWindow.toggleMaximize();

  } else if(e.key == 'F11') {
    if(await appWindow.isFullscreen()) {
      appWindow.setDecorations(true);
      appWindow.setTitle(true);
      appWindow.setFullscreen(false);
    } else {
      appWindow.setDecorations(false);
      appWindow.setTitle(false);
      appWindow.setFullscreen(true);
    }
 
  } else if(e.key == 'Escape') {
    appWindow.setFullscreen(false);
    appWindow.setDecorations(true);
    appWindow.setTitle(true);
  }
});

(function () {
  document.addEventListener('contextmenu', function(e) {
    e.preventDefault();
    e.stopPropagation();
  }, false);
})();
