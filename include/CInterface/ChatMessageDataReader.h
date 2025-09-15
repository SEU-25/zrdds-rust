#ifndef ChatMessageDataReader_h__
#define ChatMessageDataReader_h__
/*************************************************************/
/*           此文件由编译器生成，请勿随意修改                  */
/*************************************************************/

#include "ChatMessage.h"
#include "ZRDDSDataReader.h"

#ifdef __cplusplus
extern "C"
{
#endif

typedef struct ChatModule_ChatMessageDataReader ChatModule_ChatMessageDataReader;

ZRDDSUserDataReader(ChatModule_ChatMessageDataReader, ChatModule_ChatMessageSeq, ChatModule_ChatMessage);

#ifdef __cplusplus
}
#endif
#endif

