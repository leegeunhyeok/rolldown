function _class_call_check(instance, Constructor) {
  if (!(instance instanceof Constructor)) {
      throw new TypeError("Cannot call a class as a function");
  }
}
function _defineProperties(target, props) {
  for(var i = 0; i < props.length; i++){
      var descriptor = props[i];
      descriptor.enumerable = descriptor.enumerable || false;
      descriptor.configurable = true;
      if ("value" in descriptor) descriptor.writable = true;
      Object.defineProperty(target, descriptor.key, descriptor);
  }
}
function _create_class(Constructor, protoProps, staticProps) {
  if (protoProps) _defineProperties(Constructor.prototype, protoProps);
  if (staticProps) _defineProperties(Constructor, staticProps);
  return Constructor;
}
import { __exportAll, __reExport, __toCommonJS, __toESM } from 'rolldown:runtime';
var Module = /*#__PURE__*/ function() {
  "use strict";
  function Module(id) {
      _class_call_check(this, Module);
      this.exportsHolder = {
          exports: null
      };
      this.id = id;
  }
  _create_class(Module, [
      {
          key: "exports",
          get: function get() {
              return this.exportsHolder.exports;
          }
      }
  ]);
  return Module;
}();
/** @type {typeof import('./runtime-extra-dev-common-origin.js').DevRuntime} */
export var DevRuntime = /*#__PURE__*/ function() {
  "use strict";
  function DevRuntime(messenger) {
      var _this = this;
      _class_call_check(this, DevRuntime);
      this.modules = {};
      this.createEsmInitializer = function(fn, res) {
          return function() {
              return fn && (res = fn(fn = 0)), res;
          };
      };
      this.createCjsInitializer = function(cb, mod) {
          return function() {
              return mod || cb((mod = {
                  exports: {}
              }).exports, mod), mod.exports;
          };
      };
      this.__toESM = __toESM;
      this.__toCommonJS = __toCommonJS;
      this.__exportAll = __exportAll;
      this.__toDynamicImportESM = function(isNodeMode) {
          return function(mod) {
              return __toESM(mod.default, isNodeMode);
          };
      };
      this.__reExport = __reExport;
      this.sendModuleRegisteredMessage = function() {
          var cache = [];
          var timeout = null;
          var timeoutSetLength = 0;
          var self = _this;
          return function sendModuleRegisteredMessage(module) {
              if (!self.messenger) {
                  return;
              }
              cache.push(module);
              if (!timeout) {
                  timeout = setTimeout(function flushCache() {
                      if (cache.length > timeoutSetLength) {
                          timeout = setTimeout(flushCache);
                          timeoutSetLength = cache.length;
                          return;
                      }
                      self.messenger.send({
                          type: 'hmr:module-registered',
                          modules: cache
                      });
                      cache.length = 0;
                      timeout = null;
                      timeoutSetLength = 0;
                  });
                  timeoutSetLength = cache.length;
              }
          };
      }();
      this.messenger = messenger;
  }
  _create_class(DevRuntime, [
      {
          key: "createModuleHotContext",
          value: function createModuleHotContext(_moduleId) {
              throw new Error('createModuleHotContext should be implemented');
          }
      },
      {
          key: "applyUpdates",
          value: function applyUpdates(_boundaries) {
              throw new Error('applyUpdates should be implemented');
          }
      },
      {
          key: "registerModule",
          value: function registerModule(id, exportsHolder) {
              var module = new Module(id);
              module.exportsHolder = exportsHolder;
              this.modules[id] = module;
              this.sendModuleRegisteredMessage(id);
          }
      },
      {
          key: "loadExports",
          value: function loadExports(id) {
              var module = this.modules[id];
              if (module) {
                  return module.exportsHolder.exports;
              } else {
                  console.warn("Module ".concat(id, " not found"));
                  return {};
              }
          }
      }
  ]);
  return DevRuntime;
}();
