
HEADER_FMT = '''\
#ifndef {include_guard}
#define {include_guard}

/**********************************************
*
*  @file        {filename}
*
*  @date        {date}
*
*  @author      asn1tools version {version}
*
*  @brief  		Generated code for encoding/decoding data streams
*
***********************************************/

/***********************************************
* INCLUDES
***********************************************/
#include <stdbool.h>
#include "Types/eBasicTypes.h"

/***********************************************
* DATA STRUCTURES
***********************************************/
{structs}

/***********************************************
* GLOBAL FUNCTIONS DECLARATIONS
***********************************************/
{declarations}

#endif
'''

SOURCE_FMT = '''\
/**********************************************
*
*  @file        {filename}
*
*  @date        {date}
*
*  @author      asn1tools version {version}
*
*  @brief  		Generated code for encoding/decoding data streams
*
***********************************************/

/***********************************************
* INCLUDES
***********************************************/
#include <string.h>

#include "{header}"
#include "LIB/LIB_OER/eLIB_OER.h"

/***********************************************
* LOCAL FUNCTIONS DEFINITIONS
***********************************************/
{helpers}
{definitions}\
'''
