/* Memory layout of the ATSAM4E8E */
MEMORY
{
  FLASH : ORIGIN = 0x00400000, LENGTH = 512K
  RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}

panic_msg_size = 512;
stack_size = 1024;

INCLUDE cortex-m.x

