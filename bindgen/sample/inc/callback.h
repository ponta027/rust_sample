#ifndef HEADER_H
#define HEADER_H

// std::function を隠すための不透明ポインタ型
typedef void *function_handle_t;

class CallBackClass
{
public:
  CallBackClass();
  ~CallBackClass();
  void method(void);
  void set_callback(void (*callback)(int));

  void call_function(int value);

private:
  function_handle_t cbk;

  function_handle_t create_function(void (*callback)(int));

  void destroy_function();
};

#endif

