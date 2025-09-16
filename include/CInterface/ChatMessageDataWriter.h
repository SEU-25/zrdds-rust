#ifndef ChatMessageDataWriter_h__
#define ChatMessageDataWriter_h__
/*************************************************************/
/*           此文件由编译器生成，请勿随意修改                  */
/*************************************************************/

#include "ChatMessage.h"
#include "ZRDDSDataWriter.h"

#ifdef __cplusplus
extern "C"
{
#endif

typedef struct ChatModule_ChatMessageDataWriter ChatModule_ChatMessageDataWriter;

ZRDDSUserDataWriter(ChatModule_ChatMessageDataWriter, ChatModule_ChatMessage);

#ifdef __cplusplus
}
#endif
#endif

