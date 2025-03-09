#include "myclass.h"
#include <iostream>

MyClass ::MyClass()
{
}
void MyClass::method(void)
{
  std::cout << "sample" << std::endl;
  return;
}
void MyClass::method_bool(bool val)
{
  std::cout << "sample" << val << std::endl;
  return;
}

