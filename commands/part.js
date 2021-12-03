const part = (id) => {
    require(`../Day${id}/part${id}`)(id);
}

module.exports = part;
