SECTIONS
{
    .  = 0x0;
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