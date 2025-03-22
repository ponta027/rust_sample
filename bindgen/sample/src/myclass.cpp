#include "myclass.h"
#include <iostream>
MyClass ::MyClass()
{
}

MyClass ::~MyClass()
{
  std::cout << "call ~MyClass" << std::endl;
}
void MyClass::method(void)
{
  std::cout << "sample" << std::endl;
  return;
}
void MyClass::method_bool(bool val)
{
  std::cout << "method_bool:" << val << std::endl;
  return;
}
#if 0
void MyClass::method_function(std::function<void(bool,void*)> callback)
{
  std::cout << "sample" << std::endl;
  return;
}

#endif
