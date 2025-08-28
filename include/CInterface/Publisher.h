/**
 * @file:       Publisher.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef Publisher_h__
#define Publisher_h__

#include "DataWriter.h"
#include "Topic.h"
#include "PublisherQos.h"
#include "PublisherListener.h"
#include "Entity.h"
#include "ZRDDSTypeSupport.h"

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  DCPSDLL DDS_DataWriter* DDS_Publisher_create_datawriter( DDS_Publisher *publisher, DDS_Topic *topic, const DDS_DataWriterQos *writerQos, DDS_DataWriterListener*writerListener, DDS_StatusKindMask mask);
 *
 * @ingroup CPublication
 *          
 * @brief    ����DataWriter��
 *
 * @param [in,out]  publisher  ָ��Ŀ�ꡣ
 * @param    topic             ���ڹ���DataWriter��Topicʵ��ָ�롣
 * @param    writerQos         DataWriter��QoS���á�
 * @param    writerListener    ��Ҫ��װ��DataWriter��Listener��
 * @param    mask              ״̬���룬ָ����Ҫ��Listener�����״̬��
 *
 * @return  ���ܵķ���ֵ���£�
 *           - NULL��������д��ʧ�ܣ�
 *           - ����������д�ߡ�
 */

DCPSDLL DDS_DataWriter* DDS_Publisher_create_datawriter(
    DDS_Publisher *publisher,
    DDS_Topic *topic,
    const DDS_DataWriterQos *writerQos,
    DDS_DataWriterListener*writerListener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_delete_datawriter( DDS_Publisher *publisher, DDS_DataWriter *writer);
 *
 * @ingroup CPublication
 *          
 * @brief	ɾ��һ��DataWriter��
 *
 * @param [in,out]  publisher   ָ��Ŀ�ꡣ
 * @param           writer      ��Ҫ��ɾ����DataWriter��
 *
 * @return  ���ܵķ���ֵ���£�
 *          - #DDS_RETCODE_BAD_PARAMETER ����Ĳ�������
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET �����DataWriter���󣬾������ݼ���־��¼��
 *          - #DDS_RETCODE_ERROR δ������󣬾������ݼ���־��¼��
 *          - #DDS_RETCODE_OK ɾ���ɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_delete_datawriter(
    DDS_Publisher *publisher,
    DDS_DataWriter *writer);

/**
 * @fn  DCPSDLL DDS_DataWriter* DDS_Publisher_lookup_datawriter( DDS_Publisher *publisher, const DDS_Char *topicName);
 *
 * @ingroup CPublication
 *          
 * @brief    �������������Ҷ�Ӧ������д�ߡ�
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 * @param    topicName  ����������д��������������ơ�
 *
 * @return   ���ܵķ���ֵ���£�
 *           - NULLδ���ҵ�����д�ߣ�
 *           - �鵽�����׸�����д�ߡ�
 */

DCPSDLL DDS_DataWriter* DDS_Publisher_lookup_datawriter(
    DDS_Publisher *publisher,
    const DDS_Char *topicName);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_suspend_publications( DDS_Publisher *publisher);
 *
 * @ingroup CPublication
 *
 * @brief    �������ݷ�����
 *                
 * @details  �����ݷ���������֮�󣬸÷����ߴ�������������д�߷��������ݶ����ٱ�������
 *           ֱ��������ȫȡ��֮��Ż�����������ݡ�
 *           �ú������Ա���ε��ã�����ȡ������Ҳ��Ҫ���ʹ�ã��༴����ú�������ε���֮�󣬱���ȡ����ͬ�����������¿�ʼ�������ݡ�
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 *
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_NOT_ENABLED �÷�����δ��ʹ�ܣ�
 *           - #DDS_RETCODE_OK �����ɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_suspend_publications(
    DDS_Publisher *publisher);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_resume_publications( DDS_Publisher *publisher);
 *
 * @ingroup CPublication
 *
 * @brief    �ָ����ݷ������� #DDS_Publisher_suspend_publications ���ʹ�á�
 *           
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 *                  
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_NOT_ENABLED �÷�����δ��ʹ�ܣ�
 *           - #DDS_RETCODE_PRECONDITION_NOT_MET �÷�����δ���ù� #DDS_Publisher_suspend_publications ��
 *           - #DDS_RETCODE_OK �����ɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_resume_publications(
    DDS_Publisher *publisher);

#ifdef _ZRDDS_INCLUDE_PRESENTATION_QOS

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_begin_coherent_changes( DDS_Publisher *publisher);
 *
 * @ingroup CPublication
 *
 * @brief    ��ʼ���С����������޸ġ�
 *                  
 * @details  ��������д�������ķ����ߵ�QoS #DDS_PresentationQosPolicy::coherent_access == true��ʹ�ô˺�����ʼ���С�ԭ�ӡ�������
 *           �ڸú�������֮��ֱ�� #DDS_Publisher_end_coherent_changes ������֮ǰ�������������ݻᱻ���ն�һ���Է��ʵ���
 *           �༴�� #DDS_Publisher_end_coherent_changes ������֮ǰ�������ύ�����ݶ��ڽ��ն���˵���ǲ��ɷ��ʵġ�
 *           ���� #DDS_Publisher_end_coherent_changes ����֮�󣬽��ն˻��յ�һ�����ݡ�
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 *                  
 * @return	���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_NOT_ENABLED ������δʹ�ܣ�
 *           - #DDS_RETCODE_ERROR δ�������
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_begin_coherent_changes(
    DDS_Publisher *publisher);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_end_coherent_changes( DDS_Publisher *publisher);
 *
 * @ingroup CPublication
 *
 * @brief    ���������������޸ģ�ʹ���ն˿��Է����޸ĵ�ֵ�������� #DDS_Publisher_begin_coherent_changes ����֮����á�
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 *                  
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_NOT_ENABLED ������δʹ�ܡ�
 *           - #DDS_RETCODE_PRECONDITION_NOT_MET ������δ���ù� #DDS_Publisher_begin_coherent_changes ��
 *           - #DDS_RETCODE_ERROR δ�������
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_end_coherent_changes(
    DDS_Publisher *publisher);

#endif /* _ZRDDS_INCLUDE_PRESENTATION_QOS */

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_wait_for_acknowledgments( DDS_Publisher *publisher, const DDS_Duration_t *maxWait);
 *
 * @ingroup CPublication
 *
 * @brief    �������øú������߳�ֱ���÷����ߴ���������д�߷��͵��������ݶ������ն�����Ӧ���߳�ʱ��
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 * @param    maxWait    �ú����������ʱ�䡣
 *
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_NOT_ENABLED ������δʹ�ܡ�
 *           - #DDS_RETCODE_TIMEOUT �ȴ���ʱ��
 *           - #DDS_RETCODE_ERROR δ�������
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_wait_for_acknowledgments(
    DDS_Publisher *publisher,
    const DDS_Duration_t *maxWait);

/**
 * @fn  DCPSDLL DDS_DomainParticipant* DDS_Publisher_get_participant( DDS_Publisher *publisher);
 *
 * @ingroup CPublication
 *
 * @brief    ��ȡ�����÷����ߵ�������ߡ�
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 *                  
 * @return   ���ܵķ���ֵ���£�
 *           - ������Publisher��DomainParticipant��
 */

DCPSDLL DDS_DomainParticipant* DDS_Publisher_get_participant(
    DDS_Publisher *publisher);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_delete_contained_entities( DDS_Publisher *publisher);
 *
 * @ingroup CPublication
 *
 * @brief    ɾ��Publisher������������DataWriter��
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 *                  
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_ERROR δ�������
 *           - #DDS_RETCODE_OK ɾ���ɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_delete_contained_entities(
    DDS_Publisher *publisher);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_default_datawriter_qos( DDS_Publisher *publisher, const DDS_DataWriterQos *writerQos);
 *
 * @ingroup CPublication
 *
 * @brief    ��������д�ߵ�Ĭ��QoS��
 *
 * @details  ����������д��ʱ����ʹ�� #DDS_DATAWRITER_QOS_DEFAULT ֵ��ΪDataWriterQoS���롣
 *           ����û�ʹ���� #DDS_DATAWRITER_QOS_DEFAULT �������QoS���ý��ɸú�������ʱ�����QoS������
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 * @param    writerQos ��Ҫ���õ�����д��QoS�����ʹ��DEFAULT_DATAWRITER_QOS��Ϊ�������øú��������õ�Ĭ��QoS�������á�
 *
 * @return   �μ� #FooDataWriter_set_qos ��
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_default_datawriter_qos(
    DDS_Publisher *publisher,
    const DDS_DataWriterQos *writerQos);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_get_default_datawriter_qos( DDS_Publisher *publisher, DDS_DataWriterQos *writerQos);
 *
 * @ingroup CPublication
 *
 * @brief    ��ȡ�� #DDS_Publisher_set_default_datawriter_qos ���õ�DataWriterQos��
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 * @param [in,out]   writerQos ��ȡ��������д��QoS
 *
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_ERROR δ�������
 *           - #DDS_RETCODE_OK ��ȡ�ɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_get_default_datawriter_qos(
    DDS_Publisher *publisher,
    DDS_DataWriterQos *writerQos);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_copy_from_topic_qos( DDS_DataWriterQos *writerQos, const DDS_TopicQos *topicQos);
 *
 * @ingroup CPublication
 *
 * @brief    ʹ��TopicQos�еĶ�Ӧ�ֵDataWriterQos��
 *
 * @param    topicQos             ����QoS����Ϊ����������Դ��
 * @param [in,out]   writerQos    ����д��QoS�����濽�������
 *
 * @return	���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_ERROR δ�������
 *           - #DDS_RETCODE_OK ���óɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_copy_from_topic_qos(
    DDS_DataWriterQos *writerQos,
    const DDS_TopicQos *topicQos);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_qos( DDS_Publisher *publisher, const DDS_PublisherQos *publisherQos);
 *
 * @ingroup CPublication
 *
 * @brief    ���÷�����QoS��
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 * @param    publisherQos �����õķ�����QoS������ʹ�� #DDS_PUBLISHER_QOS_DEFAULT ��Ϊ������ʹ������������б����Ĭ�Ϸ�����QoS
 *
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_IMMUTABLE_POLICY ������ʹ�ܺ��޸ĵ�QoS��
 *           - #DDS_RETCODE_INCONSISTENT QoS���ڳ�ͻ��
 *           - #DDS_RETCODE_BAD_PARAMETER QoS���ڲ��Ϸ���ֵ��
 *           - #DDS_RETCODE_ERROR δ�������
 *           - #DDS_RETCODE_OK ���óɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_qos(
    DDS_Publisher *publisher,
    const DDS_PublisherQos *publisherQos);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_get_qos( DDS_Publisher *publisher, DDS_PublisherQos *publisherQos);
 *
 * @ingroup CPublication
 *
 * @brief    ��ȡ������QoS��
 *
 * @param [in,out]  publisher     ָ��Ŀ�ꡣ
 * @param [in,out]   publisherQos �����ȡ���ķ�����QoSֵ��
 *
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_OK ��ȡ�ɹ���
 *           - #DDS_RETCODE_ERROR δ�������
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_get_qos(
    DDS_Publisher *publisher,
    DDS_PublisherQos *publisherQos);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_listener( DDS_Publisher *publisher, DDS_PublisherListener *publisherListener, DDS_StatusKindMask mask);
 *
 * @ingroup CPublication
 *
 * @brief    ���÷����ߵļ�������
 *
 * @details  �����µļ���������ʹԭ�еļ��������ͷţ��û���Ҫ�Լ��������������ķ�����ͷš�
 *
 * @param [in,out]  publisher   ָ��Ŀ�ꡣ
 * @param    publisherListener  �û��ṩ�ļ��������󣬿��Դ���NULL���������
 * @param    mask        ״̬���룬ָ����Ҫ�������������״̬��
 *
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_OK ���óɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_listener(
    DDS_Publisher *publisher,
    DDS_PublisherListener *publisherListener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_PublisherListener* DDS_Publisher_get_listener( DDS_Publisher *publisher);
 *
 * @ingroup CPublication
 *
 * @brief    ��ȡ�����ߵ�ǰ�ļ�������
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 *                  
 * @return   ���ܵķ���ֵ���£�
 *           - NULL��ʾδ���ü�������
 *           - �ǿձ�ʾӦ��ͨ�� #DDS_Publisher_set_listener �����ڴ���ʱ���õļ���������
 */

DCPSDLL DDS_PublisherListener* DDS_Publisher_get_listener(
    DDS_Publisher *publisher);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_enable( DDS_Publisher *publisher);
 *
 * @ingroup CPublication
 *
 * @brief   �ֶ�ʹ�ܸ�ʵ�壬�μ�@ref entity-enable ��
 *
 * @param [in,out]  publisher        ָ��Ŀ�ꡣ
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK ����ʾ��ȡ�ɹ���
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET ����ʾ�����ĸ�ʵ����δʹ��
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_enable(
    DDS_Publisher *publisher);

/**
 * @fn  DCPSDLL DDS_Entity* DDS_Publisher_as_entity(DDS_Publisher* publisher);
 *
 * @ingroup CPublication
 *
 * @brief   ��������ת��Ϊ�����ࡱʵ�����
 *
 * @param [in,out]  publisher    ָ��Ŀ�ꡣ
 *
 * @return  �ձ�ʾת��ʧ�ܣ�����ָ�򡰸��ࡱʵ�����
 */

DCPSDLL DDS_Entity* DDS_Publisher_as_entity(DDS_Publisher* publisher);

/**
 * @struct DDS_PublisherSeq 
 *
 * @ingroup CPublication
 *
 * @brief   ���� DDS_Publisher ָ����������ͣ��μ� #DDS_USER_SEQUENCE_C ��
 */
DDS_SEQUENCE_C(DDS_PublisherSeq, DDS_Publisher*);

#ifdef _ZRXMLINTERFACE

#ifdef _ZRXMLENTITYINTERFACE

/**
 * @fn  DDS_ReturnCode_t DDS_Publisher_lookup_named_datawriters( DDS_Publisher* self, const char* pattern, DDS_StringSeq* writer_names);
 *
 * @ingroup CPublication
 *
 * @brief �������Ʒ���pattern�޶�������д������
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param pattern               ����ģʽ������֧��*��?��*�������������������ַ���?�������ⵥ���ַ�
 * @param [in,out] writer_names ���ҵõ�����д�������б�
 *
 * @return   ���ܵķ���ֵ���£�
 *           - #DDS_RETCODE_BAD_PARAMETER �������ڴ���
 *           - #DDS_RETCODE_OK ���óɹ���
 */

DCPSDLL DDS_ReturnCode_t DDS_Publisher_lookup_named_datawriters(
    DDS_Publisher* self,
    const char* pattern,
    DDS_StringSeq* writer_names);

DCPSDLL DDS_DataWriter* DDS_Publisher_lookup_datawriter_by_name(
    DDS_Publisher* self, const DDS_Char* name);

DCPSDLL const DDS_Char* DDS_Publisher_get_entity_name(
    DDS_Publisher* self);

DCPSDLL DDS_DomainParticipant* DDS_Publisher_get_factory(
    DDS_Publisher* self);

/**
 * @fn  DDS_DataWriter* DDS_Publisher_create_datawriter_from_xml_string( DDS_Publisher* self, const DDS_Char* xml_content);
 *
 * @ingroup CPublication
 *
 * @brief ��XML����һ������д�ߣ�XML���ڵ�Ϊdata_writer
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param xml_content XML�ַ���
 *
 * @return  ���ܵķ���ֵ���£�
 *           - NULL��������д��ʧ�ܣ�
 *           - ����������д�ߡ�
 */

DCPSDLL DDS_DataWriter* DDS_Publisher_create_datawriter_from_xml_string(
    DDS_Publisher* self,
    const DDS_Char* xml_content);

DCPSDLL DDS_ReturnCode_t DDS_Publisher_to_xml(
    DDS_Publisher* self,
    const DDS_Char** result,
    DDS_Boolean contained_qos);

#endif /* _ZRXMLENTITYINTERFACE */

#ifdef _ZRXMLQOSINTERFACE

/**
 * @fn  DCPSDLL DDS_DataWriter* DDS_Publisher_create_datawriter_with_topic_and_qos_profile( DDS_Publisher* self, const DDS_Char* topicName, DDS_TypeSupport* typeSupport, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_DataWriterListener* dwListener, DDS_StatusKindMask mask);
 *
 * @ingroup  CPublication
 *
 * @brief   ����ָ���������Ƶ�����д�ߣ����������ƹ���������δ����ʱ�����Զ������� ���������Ѿ����������ⴴ������д�ߡ�
 *
 * @param [in,out]  self        ָ�������ߡ�
 * @param   topicName           ����д�߹������������ơ�
 * @param [in,out]  typeSupport ����д�߹������������͵�����֧��ȫ�ֶ����ַ��DDS��Ϊÿ���������;�����һ��ȫ�ֶ��󣬶������ƹ���Ϊ�� ��������TypeSupport_instance �����㿽�����ͣ� DDS_ZeroCopyBytesTypeSupport_instance ��
 * @param   library_name        QoS������֣�������ΪNULL��
 * @param   profile_name        QoS���õ����֣�������ΪNULL��
 * @param   qos_name            QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 * @param [in,out]  dwListener  ����д�ߵļ�������
 * @param   mask                ���������롣
 *
 * @return  NULL��ʾʧ�ܣ����򷵻�����д��ָ�롣
 */

DCPSDLL DDS_DataWriter* DDS_Publisher_create_datawriter_with_topic_and_qos_profile(
    DDS_Publisher* self,
    const DDS_Char* topicName,
    DDS_TypeSupport* typeSupport,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_DataWriterListener* dwListener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_DataWriter* DDS_Publisher_create_datawriter_with_qos_profile( DDS_Publisher* self, DDS_Topic* topic, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_DataWriterListener *dw_listener, DDS_StatusKindMask mask);
 *
 * @ingroup CPublication
 *
 * @brief   ��QoS�ֿ��л�ȡ����д��Qos�����䴴������д��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in]  topic       ����������
 * @param   library_name    QoSLibrary����
 * @param   profile_name    QoSProfile����
 * @param   qos_name        QoS����
 * @param [in]  dw_listener �û��ṩ�ļ���������
 * @param   mask            ״̬���룬ָ����Ҫ�������������״̬
 *
 * @return  ���ܵķ���ֵ���£�
 *           - NULL��������д��ʧ�ܣ�
 *           - ����������д�ߡ�
 */
DCPSDLL DDS_DataWriter* DDS_Publisher_create_datawriter_with_qos_profile(
    DDS_Publisher* self,
    DDS_Topic* topic,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_DataWriterListener *dw_listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_default_datawriter_qos_with_profile( DDS_Publisher* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CPublication
 *
 * @brief   ��QoS�ֿ��л�ȡ����д��QoS��������ΪĬ������д��QoS
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_default_datawriter_qos_with_profile(
    DDS_Publisher* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_qos_with_profile( DDS_Publisher* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CPublication
 *
 * @brief   ��QoS�ֿ��л�ȡ������QoS���������õ���������
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_Publisher_set_qos_with_profile(
    DDS_Publisher* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

#endif /* _ZRXMLQOSINTERFACE */

#endif /* _ZRXMLINTERFACE */

#ifdef __cplusplus
}
#endif

#endif /* Publisher_h__*/
