#ifndef ZRDDSCSimpleInterface_h__
#define ZRDDSCSimpleInterface_h__

#include "ZROSDefine.h"

#ifdef _ZRDDS_INCLUDE_SIMPLE_INTERFACE

#include "DomainParticipantFactory.h"
#include "DomainParticipant.h"
#include "ZRDDSTypeSupport.h"
#include "DataReader.h"
#include "DataWriter.h"
#include "ZRBuiltinTypesTypeSupport.h"
#include "ZRBuiltinTypesDataReader.h"
#include "ZRBuiltinTypesDataWriter.h"
#include "DataReaderListener.h"
#include "ZRSleep.h"

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  DCPSDLL DDS_DomainParticipantFactory* DDS_Init( const DDS_Char* qosFilePath, const DDS_Char* qosName);
 *
 * @ingroup SimpleInterface
 *
 * @brief   ��ָ�����ó�ʼ��DDS�м����ֻ�����������ʼ��ʱ����һ�μ��ɡ�
 *
 * @param   qosFilePath QoS�����ļ���·���������Ǿ���·����Ҳ�����ǳ�������Ŀ¼�����·����
 *                      ΪNULLʱ��Ĭ��ʹ������Ŀ¼�� ZRDDS_QOS_PROFILES.xml �����ļ���
 * @param   qosName     QoS�����ļ��й���QoS�������ƣ�DDS���� default_lib::default_profile::qosName �����QoS��ʼ��DDS�򳧣�
 *                      ����ΪNULL������ʹ�������ļ�������Ϊdefault�����á�
 *
 * @return  NULL��ʾʧ�ܣ����򷵻�DDS�򹤳�������
 */

extern DCPSDLL DDS_DomainParticipantFactory* DDS_Init(
    const DDS_Char* qosFilePath, 
    const DDS_Char* qosName);

/**
 * @fn  DCPSDLL DDS_DomainParticipant* DDS_CreateDP( const DDS_DomainId_t domainId, const DDS_Char* qos_name);
 *
 * @ingroup  SimpleInterface
 *
 * @brief   DDS����������ߣ���ָ�������QoS�������ƣ�ͬһ����ſ����ж��������ߣ�
 *          ��ҪΪʹ�ò�ͬ�ײ㴫��Э������ⴴ����ͬ��������ߣ���ͨ��SRIO��UDP��TCP��������߸�һ����
 *
 * @param   domainId    ָ����ţ���ŷ�Χ0-232
 * @param   qos_name    ָ��QoS�����ļ����������QoS�������ƣ�����ΪNULL������ʹ�������ļ�������Ϊdefault�����á�
 *
 * @return  NULL��ʾʧ�ܣ����򷵻�������ߵ�ָ�룬��ָ�����ں��������������Լ������ߡ�
 */

extern DCPSDLL DDS_DomainParticipant* DDS_CreateDP(
    const DDS_DomainId_t domainId,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_DataWriter* DDS_PubTopic( DDS_DomainParticipant* self, const DDS_Char* topicName, DDS_TypeSupport* typeSupport, const DDS_Char* qos_name, DDS_DataWriterListener* dwListener);
 *
 * @ingroup SimpleInterface
 *
 * @brief   �������⣬ָ��������ߡ�����������������QoS���������Լ���������
 *
 * @param [in,out]  self        ָ��������ߡ�
 * @param   topicName           �������ơ�
 * @param [in,out]  typeSupport ����֧�ֶ���ָ���������������ͣ�����Ϊ�� &amp;��������+TypeSupport+_instance,
 *                              ���Ϊ�㿽�������������� &amp;DDS_ZeroCopyBytesTypeSupport_instance��
 *                              ���÷��㿽������ &amp;DDS_BytesTypeSupport_instance��IDL�ļ��Զ����ɵĴ��� &amp;
 *                              ��������TypeSupport_instance����
 * @param   qos_name            ָ��QoS�����ļ�������д��QoS�������ƣ�����ΪNULL������ʹ�������ļ�������Ϊdefault�����á�
 * @param [in,out]  dwListener  �����ߵļ�������������չ����NULL���ɡ�
 *
 * @return  NULL��ʾʧ�ܣ����򷵻�����д��ָ�루����Ψһ��ʶ�����õķ����ߣ���
 */

extern DCPSDLL DDS_DataWriter* DDS_PubTopic(
    DDS_DomainParticipant* self,
    const DDS_Char* topicName,
    DDS_TypeSupport* typeSupport,
    const DDS_Char* qos_name,
    DDS_DataWriterListener* dwListener);

/**
 * @fn  extern DCPSDLL DDS_ReturnCode_t DDS_UnPubTopic(DDS_DataWriter* writer);
 *
 * @ingroup SimpleInterface
 *
 * @brief   ָ������д��ָ��ȡ��������
 *
 * @param [in,out]  writer  ָ������д��ָ�롣
 *
 * @return  #DDS_RETCODE_OK ��ʾȡ���ɹ���������ʾʧ�ܣ�
 */

extern DCPSDLL DDS_ReturnCode_t DDS_UnPubTopic(DDS_DataWriter* writer);

/**
 * @fn  extern DCPSDLL DDS_ReturnCode_t DDS_UnPubTopicWTopicName(DDS_DomainId_t domainId, const DDS_Char* topicName);
 *
 * @ingroup SimpleInterface
 *
 * @brief   ͨ�����Լ���������ȡ��������������ڴ��ڶ����������ͬ�ķ����߽�ɾ������ͬ�����ⷢ���ߡ�
 *          �ýӿڽ���ָ���������ҽ���һ���������������ʹ�ã�
 *
 * @param   domainId    ָ����š�
 * @param   topicName   ָ���������ơ�
 *
 * @return  #DDS_RETCODE_OK ��ʾȡ���ɹ���������ʾʧ�ܣ�
 */

extern DCPSDLL DDS_ReturnCode_t DDS_UnPubTopicWTopicName(DDS_DomainId_t domainId, const DDS_Char* topicName);

/**
 * @fn  DCPSDLL DDS_DataReader* DDS_SubTopic( DDS_DomainParticipant* self, const DDS_Char* topicName, DDS_TypeSupport* typeSupport, const DDS_Char* qos_name, DDS_DataReaderListener* drListener);
 *
 * @ingroup SimpleInterface
 *
 * @brief   �������⣬ָ��������ߡ����������������Լ�QoS�������ơ�
 *
 * @param [in,out]  self        ָ��������ߡ�
 * @param   topicName           �������ơ�
 * @param [in,out]  typeSupport ����֧�ֶ���ָ���������������ͣ�����Ϊ�� &amp;��������+TypeSupport+_instance,
 *                              ���Ϊ�㿽�������������� &amp;DDS_ZeroCopyBytesTypeSupport_instance��
 *                              ���÷��㿽������ &amp;DDS_BytesTypeSupport_instance��IDL�ļ��Զ����ɵĴ��� &amp;
 *                              ��������TypeSupport_instance����
 * @param   qos_name            ָ��QoS�����ļ�������д��QoS�������ƣ�����ΪNULL������ʹ�������ļ�������Ϊdefault�����á�
 * @param [in,out]  drListener  �����ߵļ�������������չ����NULL���ɡ�
 *
 * @return  NULL��ʾʧ�ܣ����򷵻����ݶ���ָ�루����Ψһ��ʶ�����õĶ����ߣ���
 */

extern DCPSDLL DDS_DataReader* DDS_SubTopic(
    DDS_DomainParticipant* self,
    const DDS_Char* topicName,
    DDS_TypeSupport* typeSupport,
    const DDS_Char* qos_name,
    DDS_DataReaderListener* drListener);

/**
 * @fn  extern DCPSDLL DDS_ReturnCode_t DDS_UnSubTopic(DDS_DataReader* reader);
 *
 * @ingroup SimpleInterface
 *
 * @brief   ָ�����ݶ���ָ��ȡ�����ġ�
 *
 * @param [in,out]  reader  ָ�����ݶ���ָ�롣
 *
 * @return  #DDS_RETCODE_OK ��ʾȡ���ɹ���������ʾʧ�ܣ�
 */

extern DCPSDLL DDS_ReturnCode_t DDS_UnSubTopic(DDS_DataReader* reader);

/**
 * @fn  extern DCPSDLL DDS_ReturnCode_t DDS_UnSubTopicWTopicName(DDS_DomainId_t domainId, const DDS_Char* topicName);
 *
 * @ingroup SimpleInterface
 *
 * @brief   ͨ�����Լ���������ȡ�����ģ�������ڴ��ڶ����������ͬ�Ķ��Ľ�ɾ������ͬ�����ⶩ�ġ�
 *          �ýӿڽ���ָ���������ҽ���һ���������������ʹ�ã�
 *
 * @param   domainId    ָ����š�
 * @param   topicName   ָ���������ơ�
 *
 * @return  #DDS_RETCODE_OK ��ʾȡ���ɹ���������ʾʧ�ܣ�
 */

extern DCPSDLL DDS_ReturnCode_t DDS_UnSubTopicWTopicName(DDS_DomainId_t domainId, const DDS_Char* topicName);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Finalize();
 *
 * @ingroup SimpleInterface
 *
 * @brief   ������Դ��
 *
 * @return  DDS_RETCODE_OK��ʾ�ɹ��������ʾ������Դ����ʧ�ܡ�
 */

extern DCPSDLL DDS_ReturnCode_t DDS_Finalize();

/**
 * @fn  DCPSDLL DDS_ZeroCopyBytes* DDS_ZeroCopyBytesCreate(DDS_ULong maximum);
 *
 * @ingroup SimpleInterface
 *
 * @brief   �����㿽�����󣬲�����󳤶ȷ���ռ䡣
 *
 * @param   maximum �㿽�������ܷ��͵ĳ��ȡ�
 *
 * @return  NULL��ʾʧ�ܣ����ܵ�ԭ��Ϊ�����ڴ�ʧ�ܣ����򷵻��㿽������
 */

extern DCPSDLL DDS_ZeroCopyBytes* DDS_ZeroCopyBytesCreate(DDS_ULong maximum);

/**
 * @fn  extern DCPSDLL void DDS_ZeroCopyBytesDestroy(DDS_ZeroCopyBytes* sample);
 *
 * @ingroup SimpleInterface
 *
 * @brief   �� DDS_ZeroCopyBytesCreate ������Ӧ�Ļ����㿽������ռ䡣
 *
 * @param [in,out]  sample  ָ���㿽������
 */

extern DCPSDLL void DDS_ZeroCopyBytesDestroy(DDS_ZeroCopyBytes* sample);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_ZeroCopyBytesWrite( DDS_DomainId_t domainId, DDS_Char* topicName, DDS_ZeroCopyBytes* sample);
 *
 * @ingroup SimpleInterface
 *
 * @brief   ��ָ��������ָ��������㿽�����ݡ�ʹ�øýӿڵ�ǰ�᣺
 *          - ָ�����������ҽ���һ��������ߣ�
 *          - ָ�����������ҽ���һ����ָ����������ķ����ߣ�
 *          - ָ���������������������Ϊ�㿽���������ͣ�
 *          ���������������������ʹ�� DDS_ZeroCopyBytesDataWriter_write �ӿ�ָ�������߷������ݡ�
 *
 * @param   domainId            ָ����š�
 * @param [in,out]  topicName   ָ���������ơ�
 * @param [in,out]  sample      ָ�����͵����ݡ�
 *
 * @return  #DDS_RETCODE_OK ��ʾ���ͳɹ��� ������ʾʧ�ܡ�
 */

extern DCPSDLL DDS_ReturnCode_t DDS_ZeroCopyBytesWrite(
    DDS_DomainId_t domainId,
    DDS_Char* topicName,
    DDS_ZeroCopyBytes* sample);

/**
 * @fn  extern DCPSDLL DDS_Long DDS_BytesWrapper(DDS_Bytes* self, const DDS_Char* buffer, const DDS_Long length);
 *
 * @ingroup  SimpleInterface
 *
 * @brief   �û���������װ��DDS_Bytes���ͣ����㿽���������������ͣ����ýӿڲ�����ռ䣬�����Ӧ�Ļ��պ�����
 *
 * @param [in,out]  self    ָ����װ��Ŀ�ꡣ
 * @param   buffer          �û��ṩ�ռ䡣
 * @param   length          �ռ䳤�ȡ�
 *
 * @return  0��ʾ�ɹ��������ʾʧ�ܡ�
 */

extern DCPSDLL DDS_Long DDS_BytesWrapper(DDS_Bytes* self, const DDS_Char* buffer, const DDS_Long length);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_BytesWrite( DDS_DomainId_t domainId, DDS_Char* topicName, const DDS_Char* content, const DDS_Long length);
 *
 * @ingroup SimpleInterface
 *
 * @brief   ��ָ��������ָ������ķ��㿽��Bytes���ݡ�ʹ�øýӿڵ�ǰ�᣺
 *          - ָ�����������ҽ���һ��������ߣ�
 *          - ָ�����������ҽ���һ����ָ����������ķ����ߣ�
 *          - ָ���������������������ΪDDS_Bytes�������ͣ�
 *          ���������������������ʹ�� DDS_BytesDataWriter_write �ӿ�ָ�������߷������ݡ�
 *
 * @param   domainId            ָ����š�
 * @param [in,out]  topicName   ָ���������ơ�
 * @param   content             �������ݻ�������
 * @param   length              ���������ȡ�
 *
 * @return  #DDS_RETCODE_OK ��ʾ���ͳɹ��� ������ʾʧ�ܡ�
 */

extern DCPSDLL DDS_ReturnCode_t DDS_BytesWrite(
    DDS_DomainId_t domainId, 
    DDS_Char* topicName, 
    const DDS_Char* content, 
    const DDS_Long length);

#ifdef __cplusplus
}

#endif

#endif /* _ZRDDS_INCLUDE_SIMPLE_INTERFACE*/

#endif /* ZRDDSCSimpleInterface_h__ */
