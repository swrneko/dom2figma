
const res = await fetch(URL);
if (!res.ok) throw new Error('Не удалось загрузить ' + URL + ': ' + res.status);
const data = await res.json();
const n = await importAll(data, OPTS);
figma.notify('Импортировано экранов: ' + n);
log('Импортировано экранов: ' + n);
