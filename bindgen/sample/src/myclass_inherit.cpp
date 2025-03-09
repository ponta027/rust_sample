#include "myclass_inherit.h"
#include <iostream>
MyClassInherit::MyClassInherit(){
}
void MyClassInherit::method(void)
{
  MyClass::method();
  std::cout << "MyClassInherit::method(void) " << std::endl;
  return ;
}

