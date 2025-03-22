#include <iostream>
#include "myclass_ns.h"


using namespace  test;
MyClass ::MyClass()
{
}
MyClass ::~MyClass()
{
  std::cout << "call ~test::MyClass" << std::endl;
}

void MyClass::method(void)
{
  std::cout << "test::MyClass::method()" << std::endl;
  return;
}
