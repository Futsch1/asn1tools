
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

//parasoft suppress item EUCHNER-CODING_RULES_7_4_4_a-3 reason "Generated code"
//parasoft suppress item EUCHNER-CODING_RULES_7_4_5_a-3 reason "Generated code"

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

//parasoft suppress item MISRA2012-RULE-8_7-4 reason "Generated code is not optimized to remove unreferenced functions"
//parasoft suppress item MISRA2012-RULE-10_5_a-4 reason "Casting enums is required during deserialization"
//parasoft suppress item MISRA2012-RULE-15_5-4 reason "Done in generated code for simplicity of functions"

//parasoft suppress item EUCHNER-CODING_RULES_7_4_9_a-3 reason "Generated code"


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
