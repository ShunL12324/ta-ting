import { Settings, X } from 'lucide-react';
import { useState } from 'react';

export function SettingsPanel() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <>
      {/* 设置按钮 */}
      <button
        onClick={() => setIsOpen(true)}
        className="fixed bottom-6 right-6 p-3 bg-gradient-to-br from-gray-800 to-gray-900 hover:from-gray-700 hover:to-gray-800 text-white rounded-xl shadow-lg transition-all duration-300 hover:scale-105 group"
        title="设置"
      >
        <Settings className="w-5 h-5 transition-transform duration-300 group-hover:rotate-90" />
      </button>

      {/* 设置面板 */}
      {isOpen && (
        <div
          className="fixed inset-0 bg-black/50 backdrop-blur-sm flex items-center justify-center z-50 p-4 animate-in fade-in duration-200"
          onClick={() => setIsOpen(false)}
        >
          <div
            className="bg-white rounded-xl shadow-2xl p-5 max-w-md w-full max-h-[90vh] overflow-y-auto animate-in slide-in-from-bottom-4 duration-300"
            onClick={(e) => e.stopPropagation()}
          >
            {/* 标题 */}
            <div className="flex items-center justify-between mb-5">
              <h2 className="text-xl font-bold text-gray-800">设置</h2>
              <button
                onClick={() => setIsOpen(false)}
                className="p-1.5 hover:bg-gray-100 rounded-lg transition-all duration-200 hover:rotate-90"
              >
                <X className="w-5 h-5 text-gray-600" />
              </button>
            </div>

            {/* 设置项 */}
            <div className="space-y-4">
              {/* 语言选择 */}
              <div>
                <label className="block text-xs font-semibold text-gray-700 mb-1.5">
                  语言
                </label>
                <select
                  className="w-full px-3 py-2 text-sm border-2 border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 bg-gray-50 hover:bg-white cursor-pointer"
                  defaultValue="zh"
                >
                  <option value="zh">中文</option>
                  <option value="en">English</option>
                  <option value="auto">自动检测</option>
                </select>
              </div>

              {/* 热键设置 */}
              <div>
                <label className="block text-xs font-semibold text-gray-700 mb-1.5">
                  全局热键
                </label>
                <div className="px-3 py-2 border-2 border-gray-200 rounded-lg bg-gradient-to-br from-gray-50 to-gray-100">
                  <div className="flex items-center gap-1.5">
                    <kbd className="px-2 py-1 bg-white text-gray-700 rounded text-xs font-semibold shadow-sm border border-gray-200">
                      Ctrl
                    </kbd>
                    <span className="text-gray-400 text-xs font-bold">+</span>
                    <kbd className="px-2 py-1 bg-white text-gray-700 rounded text-xs font-semibold shadow-sm border border-gray-200">
                      Shift
                    </kbd>
                    <span className="text-gray-400 text-xs font-bold">+</span>
                    <kbd className="px-2 py-1 bg-white text-gray-700 rounded text-xs font-semibold shadow-sm border border-gray-200">
                      D
                    </kbd>
                  </div>
                </div>
                <p className="mt-1 text-[10px] text-gray-500 flex items-center gap-1">
                  <span className="w-1 h-1 rounded-full bg-blue-500"></span>
                  按下热键开始/停止录音
                </p>
              </div>

              {/* 选项开关 */}
              <div className="space-y-2 pt-1">
                {/* 开机自启 */}
                <label className="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors cursor-pointer group">
                  <span className="text-xs font-medium text-gray-700 group-hover:text-gray-900">
                    开机自动启动
                  </span>
                  <div className="relative">
                    <input
                      type="checkbox"
                      className="peer sr-only"
                      defaultChecked={false}
                    />
                    <div className="w-9 h-5 bg-gray-300 rounded-full peer-checked:bg-blue-500 transition-all duration-200 peer-focus:ring-2 peer-focus:ring-blue-300"></div>
                    <div className="absolute left-0.5 top-0.5 w-4 h-4 bg-white rounded-full transition-all duration-200 peer-checked:translate-x-4 shadow-sm"></div>
                  </div>
                </label>

                {/* 显示转录过程 */}
                <label className="flex items-center justify-between p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors cursor-pointer group">
                  <span className="text-xs font-medium text-gray-700 group-hover:text-gray-900">
                    显示转录过程
                  </span>
                  <div className="relative">
                    <input
                      type="checkbox"
                      className="peer sr-only"
                      defaultChecked={true}
                    />
                    <div className="w-9 h-5 bg-gray-300 rounded-full peer-checked:bg-blue-500 transition-all duration-200 peer-focus:ring-2 peer-focus:ring-blue-300"></div>
                    <div className="absolute left-0.5 top-0.5 w-4 h-4 bg-white rounded-full transition-all duration-200 peer-checked:translate-x-4 shadow-sm"></div>
                  </div>
                </label>
              </div>
            </div>

            {/* 关于信息 */}
            <div className="mt-5 pt-4 border-t-2 border-gray-100">
              <div className="text-center space-y-1.5">
                <p className="text-sm font-bold text-gray-800">
                  TaTing <span className="text-xs font-normal text-gray-500">v0.1.0</span>
                </p>
                <p className="text-xs text-gray-600">AI 离线听写输入法</p>
                <div className="flex items-center justify-center gap-1.5 text-[10px] pt-1.5">
                  <span className="px-1.5 py-0.5 bg-blue-50 text-blue-600 rounded font-medium">
                    Sherpa-ONNX
                  </span>
                  <span className="px-1.5 py-0.5 bg-green-50 text-green-600 rounded font-medium">
                    完全离线
                  </span>
                  <span className="px-1.5 py-0.5 bg-purple-50 text-purple-600 rounded font-medium">
                    隐私优先
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
