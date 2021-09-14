/*
 * ! Draw an 8x8 sprite at location (x,y)
 * !
 * ! \param x: x coordinate of bottom left pixel of sprite [0:SSD1306_COLUMNS-1]
 * ! \param y: y coordinate of bottom left pixel of sprite [0:SSD1306_ROWS-1]
 * ! \param *sprite: pointer to 8x8 sprite data (const uint8_t[8])
 * !
 * ! \return 0: successful, else error:
 * !         1: x value out of range
 * !         2: y value out of range
 * !         3: I2C error during configuration
 * !         4: I2C error during data transmission
 * !
 * ! Draws an 8x8 sprite to on-chip VRAM, then the updated VRAM region to SSD1306
 */
uint16_t ssd1306_drawSprite(uint16_t x, uint16_t y, const uint8_t *const sprite) {
    // ensure pixel location is valid
    if (x >= SSD1306_COLUMNS)   return 1;
    if (y >= SSD1306_ROWS)      return 2;
 
    // determine column range: [x:x+7]
    uint8_t colStop = x + 7;
    if (colStop >= SSD1306_COLUMNS) {
        colStop = SSD1306_COLUMNS - 1;
    }
 
    // determine page range
    uint8_t pageStart = y >> 3, // y / 8 = starting page
            pageStop = y >> 3;  // y / 8 = stopping page, unless..
    if (y & 0x07)               // if y is not an integer multiple of 8,
        pageStop++;             //   two pages must be updated
 
    // update VRAM
    unsigned int i = colStop - x + 1,       // counter for iterating
                 pageOffset = y & 0x07;     // offset from bottom of page
    while (i!=0) {
        i--;                                              // decrement counter
        uint8_t lowerPage = sprite[i] << pageOffset;      // move sprite 'up'
        ssd1306_vram[pageStart][x+i] |= lowerPage;        // OR into VRAM
        if (pageStop < SSD1306_ROWS / 8) {                      // only update second page if valid
            uint8_t upperPage = sprite[i] >> (8-pageOffset);    // move sprite 'down'
            ssd1306_vram[pageStop][x+i] |= upperPage;           // OR into VRAM
        }
    }
 
    // send configuration message
    const uint8_t configMsg[] = {
        SSD1306_CMD_START,          // start commands
        SSD1306_SETPAGERANGE,       // set page range:
        pageStart,                  //   y / 8
        pageStop,                   //
        SSD1306_SETCOLRANGE,        // set column range:
        x,                          //   x
        colStop                     //   min(x+7, 127)
    };
    if (i2c_tx(SSD1306_I2C_ADDRESS, configMsg, sizeof configMsg))   return 3;
 
    // draw updated VRAM to screen
    uint8_t dataMsg[9] = {  // message can be a max of 9 bytes
        SSD1306_DATA_START  // start data
    };
    i = pageStart;
    while (i != pageStop + 1) {                 // loop over pages
        if (i < SSD1306_ROWS / 8) {                 // only if valid page
            unsigned int j = colStop - x + 1;           // local counter to
            while (j != 0) {                            // copy VRAM into dataMsg
                j--;
                dataMsg[j+1] = ssd1306_vram[i][x+j];
            }
            if (i2c_tx(SSD1306_I2C_ADDRESS, dataMsg, colStop-x+2))  return 4;
        }
        i++;
    }
 
    // return successful
    return 0;
}
