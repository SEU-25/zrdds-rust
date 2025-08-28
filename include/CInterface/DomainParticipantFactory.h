/**
 * @file:       DomainParticipantFactory.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef DomainParticipantFactory_h__
#define DomainParticipantFactory_h__

#include "DomainParticipant.h"
#include "DomainParticipantFactoryQos.h"
#include "DomainParticipantQos.h"
#include "DomainParticipantListener.h"
#include "DomainId_t.h"
#include "ZRDynamicData.h"

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  DCPSDLL DDS_DomainParticipantFactory* DDS_DomainParticipantFactory_get_instance();
 *
 * @ingroup CDomain
 *
 * @brief   ��ȡ������ߵ�������
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - NULL��ʾ������������ʧ�ܣ����ܵ�ԭ��
 *              - ��ʼ����Դʧ�ܣ���ϸ�μ���־��
 *              - Mac����ʱ����֤ʧ�ܣ�
 *          - ��ȡ�ɹ��ĵ������� ������Ӧ�ó����һ�λ�ȡʵ��ʱ��Ӧ�������µļ�飺
 *          @code{cpp}
 *          if (DDS_DomainParticipantFactory_get_instance() == NULL)
 *          {
 *              // get domainparticipant factory faild.
 *          }
 *          @endcode
 *          
 * @note    �÷����̲߳���ȫ������߳�ͬʱ����ʵ������ȫ����ȡʵ����ȫ��
 */

DCPSDLL DDS_DomainParticipantFactory* DDS_DomainParticipantFactory_get_instance();

/**
 * @fn  DCPSDLL DDS_DomainParticipantFactory* DDS_DomainParticipantFactory_get_instance_w_qos(const DDS_DomainParticipantFactoryQos* qoslist);
 *
 * @ingroup CDomain
 *          
 * @brief   ʹ��ָ����DomainParticipantFactoryQos����ȡ������߹����ĵ�������.
 *
 * @param   qoslist ָ����DomainParticipantFactoryQos.
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - NULL��ʾ������������ʧ�ܣ����ܵ�ԭ��
 *              - ��ʼ����Դʧ�ܣ���ϸ�μ���־��
 *              - Mac����ʱ����֤ʧ�ܣ�
 *          - ��ȡ�ɹ��ĵ�������  
 *
 * @note    �÷����̲߳���ȫ������߳�ͬʱ����ʵ������ȫ����ȡʵ����ȫ��.
 *
 */

DCPSDLL DDS_DomainParticipantFactory* DDS_DomainParticipantFactory_get_instance_w_qos(const DDS_DomainParticipantFactoryQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_finalize_instance();
 *
 * @ingroup CDomain
 *
 * @brief   �����������÷���ͬ�����̲߳���ȫ�ģ�����߳�ͬʱ���øú��������ܻ�����⡣
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ�����ɹ���
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :�ɸ�������߹����������������δɾ���ꣻ
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_finalize_instance();

/**
 * @fn  DCPSDLL DDS_DomainParticipant* DDS_DomainParticipantFactory_create_participant( DDS_DomainParticipantFactory* self, const DDS_DomainId_t domainId, const DDS_DomainParticipantQos* qoslist, DDS_DomainParticipantListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CDomain
 *
 * @brief   ����һ���µ��������ʵ�壬������QoS�Լ���������������ߵĴ�������Ӧ�ó���������@e domainId ָ�������н���ͨ�š�
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param   domainId            ������Ҫ�������ţ�ȡֵ��ΧΪ[0-232]��
 * @param   qoslist             ��ʾΪ������������õ�QoS�� #DDS_DOMAINPARTICIPANT_QOS_DEFAULT ����ʹ��������߹����б����Ĭ�ϵ�QoS��
 * @param [in,out]  listener    Ϊ������������õļ�������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  �����ɹ�ָ�򴴽��ɹ����������ʵ����󣬷��򷵻�NULL��ʧ�ܵ�ԭ�����Ϊ��
 *          - ����ռ�ʧ�ܻ��߳�ʼ����Դʧ�ܣ�����Ĵ�����Ϣ�μ���־��
 *          - @e qoslist ������Чֵ���ߺ��в�һ�µ�QoS��
 */

DCPSDLL DDS_DomainParticipant* DDS_DomainParticipantFactory_create_participant(
    DDS_DomainParticipantFactory* self,
    const DDS_DomainId_t domainId,
    const DDS_DomainParticipantQos* qoslist,
    DDS_DomainParticipantListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_delete_participant( DDS_DomainParticipantFactory* self, DDS_DomainParticipant* dp);
 *
 * @ingroup CDomain
 *
 * @brief   �÷���ɾ��ָ����������ߡ��ڵ��ø÷���֮ǰ��Ҫ��֤��������ߵ�������ʵ�嶼�Ѿ���ɾ�������򽫻᷵�ش���
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  dp      ָ����Ҫɾ�����������ʵ�塣
 *
 * @return  ���ܵķ���ֵ���£�
 *          - #DDS_RETCODE_OK :��ʾɾ���ɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER :�û��ṩ�Ĳ���������Ч��������߶��󣬰���NULLֵ��
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :ָ����������߲�����ɾ����������������ʵ��δɾ����
 *          - #DDS_RETCODE_ERROR :ָ����������߲����ɹ���������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_delete_participant(
    DDS_DomainParticipantFactory* self, 
    DDS_DomainParticipant* dp);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_delete_contained_entities( DDS_DomainParticipantFactory* self);
 *
 * @ingroup CDomain
 *
 * @brief   ɾ�����е�������߼�����ʵ�壬�ú���������ɾ����������ɾ����������ʵ�塣
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 *
 * @return  ���ܵķ���ֵ���£�
 *          - #DDS_RETCODE_OK :��ʾɾ���ɹ���
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :ĳЩ������߲�����ɾ����������������ʵ��δɾ����
 *          - #DDS_RETCODE_ERROR :ָ����������߲����ɹ���������.
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_delete_contained_entities(
    DDS_DomainParticipantFactory* self);

/**
 * @fn  DCPSDLL DDS_DomainParticipant* DDS_DomainParticipantFactory_lookup_participant( DDS_DomainParticipantFactory* self, DDS_DomainId_t domainId);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�����ָ����@e domainId �²���������ߣ�����ж��������ߣ��򷵻�����һ����
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   domainId    ��Ҫ���ҵ���
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - NULL��ʾ����ʧ�ܣ��������ڸ����µ�ǰû��������ߣ�
 *          - �ǿձ�ʾ���ҵĽ�������صĽ��Ϊ�������������ָ����С��������ߡ�
 */

DCPSDLL DDS_DomainParticipant* DDS_DomainParticipantFactory_lookup_participant(
    DDS_DomainParticipantFactory* self, 
    DDS_DomainId_t domainId);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_set_default_participant_qos( DDS_DomainParticipantFactory* self, const DDS_DomainParticipantQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷������øù���Ϊ������߱����Ĭ��QoS���á�
 *
 * @details Ĭ�ϵ�QoS�ڴ����µ��������ʱָ��QoS����Ϊ #DDS_DOMAINPARTICIPANT_QOS_DEFAULT ʱʹ�õ�QoS���ã�
 *          ʹ�������ֵ #DDS_DOMAINPARTICIPANT_QOS_DEFAULT �������QoS�еĸ������õ�����ΪĬ��ֵ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   qoslist ָ��QoS���á�
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER :��ʾ@e qoslistΪ�գ�����@e qoslist ������Чֵ��
 *          - #DDS_RETCODE_INCONSISTENT :��ʾ@e qoslist �о��в����ݵ����ã�
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ����縴��QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_set_default_participant_qos(
    DDS_DomainParticipantFactory* self, 
    const DDS_DomainParticipantQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_default_participant_qos( DDS_DomainParticipantFactory* self, DDS_DomainParticipantQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�����ȡ�ù���Ϊ������߱����Ĭ��QoS���á�
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  qoslist ���ڲ�����ʾ��ȡ�Ľ��.
 *
 * @return  ��ǰ�ķ���ֵ���ͣ�
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ����縴��QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_default_participant_qos(
    DDS_DomainParticipantFactory* self, 
    DDS_DomainParticipantQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_set_qos( DDS_DomainParticipantFactory* self, const DDS_DomainParticipantFactoryQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�������������߹��������QoS�������ֵ #DDS_DOMAINPARTICIPANT_FACTORY_QOS_DEFAULT ��ʾ����ΪĬ��ֵ��
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param   qoslist ָ��Ŀ��QoS��
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_BAD_PARAMETER :��ʾ@e qoslistΪ�գ�����@e qoslist ������Чֵ��
 *          - #DDS_RETCODE_INCONSISTENT :��ʾ@e qoslist �о��в����ݵ����ã�
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ����縴��QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_set_qos(
    DDS_DomainParticipantFactory* self, 
    const DDS_DomainParticipantFactoryQos* qoslist);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_qos( DDS_DomainParticipantFactory* self, DDS_DomainParticipantFactoryQos* qoslist);
 *
 * @ingroup CDomain
 *
 * @brief   �÷�����ȡΪ������߹������õ�QoS��
 *
 * @details ������ø÷���֮ǰδ���ù� #DDS_DomainParticipantFactory_set_qos ���򷵻�ϵͳĬ�ϵ�QoS���ã�
 *          ���򷵻� #DDS_DomainParticipantFactory_set_qos �Ľ����
 *
 * @param [in,out]  self    ָ��Ŀ�ꡣ
 * @param [in,out]  qoslist ���ڲ������洢��ȡ�Ľ����
 *
 * @return  ��ǰ�ķ���ֵ���ͣ�
 *          - #DDS_RETCODE_OK :��ʾ���óɹ���
 *          - #DDS_RETCODE_ERROR :��ʾʧ�ܣ����縴��QoSʱ��������
 */

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_qos(
    DDS_DomainParticipantFactory* self, 
    DDS_DomainParticipantFactoryQos* qoslist);

#ifdef _ZRXMLINTERFACE

#ifdef _ZRXMLENTITYINTERFACE

DCPSDLL DDS_DomainParticipantFactory* DDS_DomainParticipantFactory_new_instance(
    DDS_DomainParticipantFactoryQos* qos);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_delete_instance(
    DDS_DomainParticipantFactory* self);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_add_to_type_library_from_xml_string(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* xml_content);

DCPSDLL TypeCodeHeader* DDS_DomainParticipantFactory_lookup_type_by_name(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* name);

DCPSDLL ZRDynamicData* DDS_DomainParticipantFactory_gen_dynamic_data(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* type_name,
    const DDS_Char* xml_content);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_lookup_named_participants(
    DDS_DomainParticipantFactory* self,
    const char* pattern,
    DDS_StringSeq* participant_names);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_lookup_named_types(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* pattern,
    DDS_Long reference_depth,
    DDS_StringSeq* participant_names);

DCPSDLL DDS_Entity* DDS_DomainParticipantFactory_lookup_entity_by_name(
    DDS_DomainParticipantFactory* self, const DDS_Char* name);

DCPSDLL DDS_DomainParticipant* DDS_DomainParticipantFactory_lookup_participant_by_name(
    DDS_DomainParticipantFactory* self, const DDS_Char* name);

DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_type_to_xml(
    DDS_DomainParticipantFactory* self, const DDS_Char* type_name, const DDS_Char** result);

DCPSDLL DDS_DomainParticipant* DDS_DomainParticipantFactory_create_participant_from_xml_string(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* xml_content);

#endif /* _ZRXMLENTITYINTERFACE */

#ifdef _ZRXMLQOSINTERFACE

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_add_qos_library( DDS_DomainParticipantFactory* self, const DDS_Char* xml_representation);
 *
 * @ingroup CDomain
 *
 * @brief   ���һ��QoS��
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   xml_representation  ��XML�ַ�����ʾ��QoS�⡣
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ��ӳɹ���
 *         - #DDS_RETCODE_BAD_PARAMETER :��ʾ�������ڴ�����ȱʧ������
 *         - #DDS_RETCODE_ERROR :��ʾ���ʧ�ܡ�
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_add_qos_library(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* xml_representation);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_qos_library_to_xml( DDS_DomainParticipantFactory* self, const DDS_Char* qos_library_name, const DDS_Char** result);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS��ת��ΪXML
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   qos_library_name    QoS������֣�������ΪNULL��
 * @param [out] result          ת���õ��Ľ��
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾת���ɹ���
 *         - #DDS_RETCODE_ERROR :��ʾת��ʧ�ܡ�
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_qos_library_to_xml(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* qos_library_name,
    const DDS_Char** result);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_lookup_qos_libraries( DDS_DomainParticipantFactory* self, const DDS_Char* pattern, DDS_StringSeq* qos_library_names);
 *
 * @ingroup CDomain
 *
 * @brief   �������Ʒ���pattern�޶���QoS��
 *
 * @param [in,out]  self                ָ��Ŀ��
 * @param   pattern                     ����ģʽ������֧��*��?��*�������������������ַ���?�������ⵥ���ַ���
 * @param [in,out]  qos_library_names   ���ҵõ���QoS�������б�
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���ҳɹ���
 *         - #DDS_RETCODE_BAD_PARAMETER :��ʾ�������ڴ�����ȱʧ������
 *         - #DDS_RETCODE_ERROR :��ʾ����ʧ�ܡ�
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_lookup_qos_libraries(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* pattern,
    DDS_StringSeq* qos_library_names);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_remove_qos_library( DDS_DomainParticipantFactory* self, const DDS_Char* qos_library_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��ָ��QoS�����Ƴ�һ��QoS����
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   qos_library_name    ��Ҫ���Ƴ���QoS�������
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ�Ƴ��ɹ���
 *         - #DDS_RETCODE_BAD_PARAMETER :��ʾ�������ڴ�����ȱʧ������
 *         - #DDS_RETCODE_ERROR :��ʾ�Ƴ�ʧ�ܡ�
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_remove_qos_library(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* qos_library_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_add_qos_profile( DDS_DomainParticipantFactory* self, const DDS_Char* qos_library_name, const DDS_Char* xml_representation);
 *
 * @ingroup CDomain
 *
 * @brief   ��ָ��QoS�������һ��QoS����
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   qos_library_name    ��Ҫ���QoS���õ�QoS�⡣
 * @param   xml_representation  ��XML��ʾ��QoS�������ݡ�
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ��ӳɹ���
 *         - #DDS_RETCODE_BAD_PARAMETER :��ʾ�������ڴ�����ȱʧ������
 *         - #DDS_RETCODE_ERROR :��ʾ���ʧ�ܡ�
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_add_qos_profile(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* qos_library_name,
    const DDS_Char* xml_representation);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_reload_qos_profiles( DDS_DomainParticipantFactory* self);
 *
 * @ingroup CDomain
 *
 * @brief   ����QosProfileQosPolicy���й�QoS���õ����ã��������¼���QoS���õ����У���DomainParticipantFactory��ʼ��ʱ��ʽ���ã�
 *          �쳣�����߼���
 *          - �����õ�·��������ʱ�����Ը�������ʾ�û�������RETCODE_OK
 *          - �����õ�XML���ڲ��ɺ��ԵĴ���ʱ��XML��ʽ����ȣ�,��ʾ�û���������RETCODE_ERROR.
 *        ���е�UserData��GroupData��TopicData����֧��ʹ��String��ʽ��Sequence��ʽ����
 *          - String��ʽΪ���ַ���ֱ����Ϊvalue�ڵ��text
 *          - Sequence��ʽΪ��value����Sequenceʹ�ã���������sequenceMaxLength���ԣ�������ÿ��Ԫ����Ϊitemд��
 *
 * @param [in,out]  self    ָ��Ŀ��
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ������õ�·�������ڣ�
 *         - #DDS_RETCODE_BAD_PARAMETER ����ʾ��������ȷ���µ����QoS�����
 *         - #DDS_RETCODE_ERROR :��ʾXML���ڴ�����QoS�����ʧ�ܣ�
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_reload_qos_profiles(
    DDS_DomainParticipantFactory* self);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_lookup_qos_profiles( DDS_DomainParticipantFactory* self, const DDS_Char* qos_library_name, const DDS_Char* pattern, DDS_StringSeq* qos_profile_names);
 *
 * @ingroup CDomain
 *
 * @brief   ��ָ��QoS���в������ַ���pattern��QoS����
 *
 * @param [in,out]  self                ָ��Ŀ��
 * @param   qos_library_name            QoS������֣�������ΪNULL��
 * @param   pattern                     ����ģʽ������֧��*��?��*�������������������ַ���?�������ⵥ���ַ�
 * @param [in,out]  qos_profile_names   ���ҵõ���QoS���������б�
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���ҳɹ���
 *         - #DDS_RETCODE_BAD_PARAMETER :��ʾ�������ڴ�����ȱʧ������
 *         - #DDS_RETCODE_ERROR :��ʾ����ʧ�ܡ�
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_lookup_qos_profiles(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* qos_library_name,
    const DDS_Char* pattern,
    DDS_StringSeq* qos_profile_names);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_qos_profile_to_xml( DDS_DomainParticipantFactory* self, const DDS_Char* qos_library_name, const DDS_Char* qos_profile_name, const DDS_Char** result);
 *
 * @ingroup CDomain
 *
 * @brief   ��һ��QoS����ת��ΪXML
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   qos_library_name    QoS�������
 * @param   qos_profile_name    QoS���õ�����
 * @param [out] result          ת���õ��Ľ��
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾת���ɹ���
 *         - #DDS_RETCODE_ERROR :��ʾת��ʧ�ܡ�
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_qos_profile_to_xml(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* qos_library_name,
    const DDS_Char* qos_profile_name,
    const DDS_Char** result);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_remove_qos_profile( DDS_DomainParticipantFactory* self, const DDS_Char* qos_library_name, const DDS_Char* qos_profile_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��ָ��QoS�����Ƴ�һ��QoS����
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   qos_library_name    QoS�������
 * @param   qos_profile_name    QoS���õ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ�Ƴ��ɹ���
 *         - #DDS_RETCODE_BAD_PARAMETER :��ʾ�������ڴ�����ȱʧ������
 *         - #DDS_RETCODE_ERROR :��ʾ�Ƴ�ʧ�ܡ�
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_remove_qos_profile(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* qos_library_name,
    const DDS_Char* qos_profile_name);

/**
 * @fn  DCPSDLL void DDS_DomainParticipantFactory_unload_qos_profiles( DDS_DomainParticipantFactory* self);
 *
 * @ingroup CDomain
 *
 * @brief   ж�����е�QoS����
 *
 * @param [in,out]  self    ָ��Ŀ��
 */
DCPSDLL void DDS_DomainParticipantFactory_unload_qos_profiles(
    DDS_DomainParticipantFactory* self);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_set_default_participant_qos_with_profile( DDS_DomainParticipantFactory* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ�������QoS��������ΪĬ���������QoS
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
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_set_default_participant_qos_with_profile(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_datareader_qos_from_profile( DDS_DomainParticipantFactory* self, DDS_DataReaderQos* qos, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ���ݶ���QoS������
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param [in,out]  qos     ��ȡ����QoS
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_datareader_qos_from_profile(
    DDS_DomainParticipantFactory* self,
    DDS_DataReaderQos* qos,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_datawriter_qos_from_profile( DDS_DomainParticipantFactory* self, DDS_DataWriterQos* qos, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ����д��QoS������
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param [in,out]  qos     ��ȡ����QoS
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_datawriter_qos_from_profile(
    DDS_DomainParticipantFactory* self,
    DDS_DataWriterQos* qos,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_participant_factory_qos_from_profile( DDS_DomainParticipantFactory* self, DDS_DomainParticipantFactoryQos* qos, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ������߹���QoS������
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param [in,out]  qos     ��ȡ����QoS
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_participant_factory_qos_from_profile(
    DDS_DomainParticipantFactory* self,
    DDS_DomainParticipantFactoryQos* qos,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_participant_qos_from_profile( DDS_DomainParticipantFactory* self, DDS_DomainParticipantQos* qos, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ�������QoS������
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param [in,out]  qos     ��ȡ����QoS
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_participant_qos_from_profile(
    DDS_DomainParticipantFactory* self,
    DDS_DomainParticipantQos* qos,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_publisher_qos_from_profile( DDS_DomainParticipantFactory* self, DDS_PublisherQos* qos, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ������QoS������
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param [in,out]  qos     ��ȡ����QoS
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_publisher_qos_from_profile(
    DDS_DomainParticipantFactory* self,
    DDS_PublisherQos* qos,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_subscriber_qos_from_profile( DDS_DomainParticipantFactory* self, DDS_SubscriberQos* qos, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ������QoS������
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param [in,out]  qos     ��ȡ����QoS
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_subscriber_qos_from_profile(
    DDS_DomainParticipantFactory* self,
    DDS_SubscriberQos* qos,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_topic_qos_from_profile( DDS_DomainParticipantFactory* self, DDS_TopicQos* qos, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡ����QoS������
 *
 * @param [in,out]  self    ָ��Ŀ��
 * @param [in,out]  qos     ��ȡ����QoS
 * @param   library_name    QoS������֣�������ΪNULL��
 * @param   profile_name    QoS���õ����֣�������ΪNULL��
 * @param   qos_name        QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *         - #DDS_RETCODE_OK :��ʾ���óɹ���
 *         - #DDS_RETCODE_ERROR :��ʾδ֪�����µ����ô���
 */
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_get_topic_qos_from_profile(
    DDS_DomainParticipantFactory* self,
    DDS_TopicQos* qos,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_set_qos_with_profile( DDS_DomainParticipantFactory* self, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡQoS���ò����õ�������߹���
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
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_set_qos_with_profile(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name);

/**
 * @fn  DCPSDLL DDS_DomainParticipant* DDS_DomainParticipantFactory_create_participant_with_qos_profile( DDS_DomainParticipantFactory* self, const DDS_DomainId_t domainId, const DDS_Char* library_name, const DDS_Char* profile_name, const DDS_Char* qos_name, DDS_DomainParticipantListener* listener, DDS_StatusKindMask mask);
 *
 * @ingroup CDomain
 *
 * @brief   ��QoS�ֿ��ȡQoS���ò����䴴���������
 *
 * @param [in,out]  self        ָ��Ŀ��
 * @param   domainId            ��Id
 * @param   library_name        QoS������֣�������ΪNULL��
 * @param   profile_name        QoS���õ����֣�������ΪNULL��
 * @param   qos_name            QoS�����֣�����ΪNULL����ת��Ϊdefault�ַ�����
 * @param [in,out]  listener    Ϊ������������õļ�������
 * @param   mask                ����Ӧ�ó������Ȥ��״̬��ֻ��Ӧ�ó������Ȥ��״̬�����仯ʱ���Ż�֪ͨӦ�ó���
 *
 * @return  ��ǰ���ܵķ���ֵ���£�
 *          - �ǿձ�ʾ����������߳ɹ���
 *          - NULL��ʾ����ʧ�ܡ�
 */
DCPSDLL DDS_DomainParticipant* DDS_DomainParticipantFactory_create_participant_with_qos_profile(
    DDS_DomainParticipantFactory* self,
    const DDS_DomainId_t domainId,
    const DDS_Char* library_name,
    const DDS_Char* profile_name,
    const DDS_Char* qos_name,
    DDS_DomainParticipantListener* listener,
    DDS_StatusKindMask mask);

/**
 * @fn  DCPSDLL DDS_DomainParticipantFactory* DDS_DomainParticipantFactory_get_instance_w_profile( const DDS_Char* qosFilePath, const DDS_Char* libName, const DDS_Char* profileName, const DDS_Char* qosName);
 *
 * @ingroup CDomain
 *
 * @brief   ��ָ��������ʼ��DDS������߹�����
 *
 * @param   qosFilePath qos�����ļ�·������ΪNULLʱ����Ĭ��ʹ������Ŀ¼�� ZRDDS_QOS_PROFILES.xml �ļ���
 * @param   libName     Qos�����ƣ�������ΪNULL��
 * @param   profileName Qos�������ƣ�������ΪNULL��
 * @param   qosName     Qos���ƣ�����Ϊ�գ���ת��Ϊdefault�ַ�����
 *
 * @return  NULL��ʾʧ�ܣ����򷵻ص���ָ�롣
 */

DCPSDLL DDS_DomainParticipantFactory* DDS_DomainParticipantFactory_get_instance_w_profile(
    const DDS_Char* qosFilePath, 
    const DDS_Char* libName, 
    const DDS_Char* profileName, 
    const DDS_Char* qosName);

#endif /* _ZRXMLQOSINTERFACE */

#endif /* _ZRXMLINTERFACE */

#ifdef _ZRDDSSECURITY
DCPSDLL DDS_ReturnCode_t DDS_DomainParticipantFactory_load_security_plugin(
    DDS_DomainParticipantFactory* self,
    const DDS_Char* plugin_name,
    const DDS_Char* file_name,
    const DDS_Char* retrieve_instance_func_name,
    const DDS_Char* finalize_instance_func_name);
#endif /* _ZRDDSSECURITY */

#ifdef __cplusplus
}
#endif

#endif /* DomainParticipantFactory_h__*/
