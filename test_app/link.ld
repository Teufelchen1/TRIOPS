SECTIONS
{
    .  = 0x20000000;
    .entry :
    {
        
    }
    .text : 
    {
        *(.text)
    }
    .rodata : 
    {
        *(.rodata)
    }
}