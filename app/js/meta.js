export function sample(str) {
    return alert(str);
}

export function updateTitle(newTitle) {
    document.title = newTitle;
}


export function updateMetaByName(metaName, value) {
    let meta = document.querySelector(`meta[name=${metaName}]`);
    if (meta) {
        meta.content = value;
    } else {
        meta = document.createElement("meta");
        meta.name = metaName;
        meta.content = value;
        document.getElementsByTagName("head")[0].appendChild(meta);
    }
}
/
