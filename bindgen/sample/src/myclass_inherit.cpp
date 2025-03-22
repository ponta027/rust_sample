#include "myclass_inherit.h"

#include <iostream>
MyClassInherit::MyClassInherit()
{
}
MyClassInherit::~MyClassInherit()
{
  std::cout << "call ~MyClassInherit" << std::endl;
}
void MyClassInherit::method(void)
{
  MyClass::method();
  std::cout << "MyClassInherit::method(void) " << std::endl;
  return;
}

