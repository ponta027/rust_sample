#include <iostream>
#include "myclass_ns.h"


using namespace  test;
MyClass ::MyClass()
{
}
void MyClass::method(void)
{
  std::cout << "test::MyClass::method()" << std::endl;
  return;
}
