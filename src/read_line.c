/**
 * @brief This file is required because when I wanted to use the
 * `stdin().read_line()` function of Rust, the entire line
 * was removed before requesting user input.
 *
 * I wrote this function to handle it much easier.
 *
 * {Limitations}
 * Although this function returns a complete string in C,
 * my Rust code only works with *one* character because
 * I did not know what the equivalent of `char*` was in
 * Rust. So I use there a `*mut ::std::os::raw::c_char`
 * which does only return the first character of the entire
 * string.
 *
 * @author avolgha
 * @version 1.0.0
 */

#include <stdio.h>
#include <stdlib.h>

char *read_line()
{
  char *line = NULL;
  size_t len = 0;
  printf("[y/n] > ");
  getline(&line, &len, stdin);
  return line;
}