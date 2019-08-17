S;
S[K];
var Store = /** @class */ (function () {
    function Store() {
        this.state = new Proxy({}, {});
    }
    Store.prototype.set = function (key, value) {
    };
    return Store;
}());
