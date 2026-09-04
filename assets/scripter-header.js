// dom2figma: импорт через плагин Scripter (https://www.figma.com/community/plugin/757836922707087381).
// 1. В папке проекта: dom2figma serve            (раздаёт out/*.json с CORS)
// 2. В Figma: Plugins → Scripter → New script → вставить этот файл → Run.
const URL = '__URL__';
const OPTS = { skipDup: true, collapse: true, separatePages: false };

const log = (t) => print(t);
