#include "callback.h"
#include <functional>
#include <iostream>
// `std::function<void(int)>` を隠蔽するための構造体
struct FunctionWrapper
{
  std::function<void(int)> func;
};

CallBackClass::CallBackClass()
{
}
CallBackClass::~CallBackClass()
{
  destroy_function();
}

void CallBackClass::method(void)
{
}
void CallBackClass::set_callback(void (*callback)(int))
{
  cbk = create_function(callback);
}

// `std::function` を作成し、そのポインタを返す
function_handle_t CallBackClass::create_function(void (*callback)(int))
{
  return new FunctionWrapper{[callback](int value) { callback(value); }};
}
// `std::function` を実行する
void CallBackClass::call_function(int value)
{
  if (cbk)
  {
    static_cast<FunctionWrapper *>(cbk)->func(value);
  }
}

// `std::function` を解放する
void CallBackClass::destroy_function()
{
  delete static_cast<FunctionWrapper *>(cbk);
}

