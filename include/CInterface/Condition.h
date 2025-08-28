/**
 * @file:       Condition.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef Condition_h__
#define Condition_h__

#include "ZRDDSCommon.h"
#include "OsResource.h"
#include "ReturnCode_t.h"
#include "ViewStateMask.h"
#include "SampleStateMask.h"
#include "InstanceStateMask.h"
#include "ZRSequence.h"
#include "StatusKindMask.h"
#include "ZRDDSCWrapper.h"

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  DCPSDLL DDS_Boolean DDS_StatusCondition_get_trigger_value(const DDS_StatusCondition* conditon);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ��ȡ״̬������ǰ�Ĵ���״̬��
 *
 * @param   conditon    statusCondition��
 *
 * @return  true��ʾ��������false��ʾ��������δ��������
 */

DCPSDLL DDS_Boolean DDS_StatusCondition_get_trigger_value(
    const DDS_StatusCondition* conditon);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_StatusCondition_set_enabled_statuses( DDS_StatusCondition* conditon, DDS_StatusKindMask enabledStatuses);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ������ķ�ʽ����״̬�������û����ĵ�״̬��
 *
 * @param [in,out]  conditon    ָ��Ŀ�ꡣ
 * @param   enabledStatuses     ���룬�û����ĵ�״̬��λ������
 *
 * @return  ��ǰ���Ƿ��� #DDS_RETCODE_OK ��
 */

DCPSDLL DDS_ReturnCode_t DDS_StatusCondition_set_enabled_statuses(
    DDS_StatusCondition* conditon, 
    DDS_StatusKindMask enabledStatuses);

/**
 * @fn  DCPSDLL DDS_StatusKindMask DDS_StatusCondition_get_enabled_statuses( const DDS_StatusCondition* conditon);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ��ȡǰһ���û�ͨ�� ::DDS_StatusCondition_set_enabled_statuses �������õĹ��ĵ�״̬��
 *
 * @param   conditon    ָ��Ŀ�ꡣ
 *
 * @return  ���뷽ʽ���ص�ǰ״̬������������״̬���ϡ�
 */

DCPSDLL DDS_StatusKindMask DDS_StatusCondition_get_enabled_statuses(
    const DDS_StatusCondition* conditon);

/**
 * @fn  DDS_Entity* DDS_StatusCondition_get_entity(DDS_StatusCondition* condition);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ��ȡ״̬����������ʵ�塣
 *
 * @param [in,out]  condition   ָ��Ŀ�ꡣ
 *
 * @return  NULL��ʾʧ�ܣ�����ָ�������ʵ��ĸ�ָ�룬�û�ͨ������ת��Ϊ�����ʵ�����͡�
 */

DCPSDLL DDS_Entity* DDS_StatusCondition_get_entity(
    DDS_StatusCondition* condition);

#ifdef _ZRDDS_INCLUDE_GUARD_CONDITION

/**
 * @fn  DCPSDLL DDS_GuardCondition* DDS_GuardCondition_new();
 *
 * @ingroup CInfrastruct
 *
 * @brief   Ĭ�Ϲ��촦�ڷǴ���״̬�ļ���������
 *
 * @return  ��NULL��ʾ�µļ���������NULL��ʾʧ�ܣ�ʧ�ܵ�ԭ�����Ϊ�����ڴ�ʧ�ܡ�
 */

DCPSDLL DDS_GuardCondition* DDS_GuardCondition_new();

/**
 * @fn  void DDS_GuardCondition_delete(DDS_GuardCondition* condition);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ɾ��ָ������������
 *
 * @param [in,out]  condition   ָ��Ŀ�ꡣ
 */

DCPSDLL void DDS_GuardCondition_delete(
    DDS_GuardCondition* condition);

/**
 * @fn  DDS_Boolean DDS_GuardCondition_get_trigger_value(const DDS_GuardCondition* conditon);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ��ȡ����������ǰ�Ĵ���״̬��
 *
 * @param   conditon    ָ��Ŀ�ꡣ
 *
 * @return  true��ʾ��������false��ʾ��������δ��������
 */

DCPSDLL DDS_Boolean DDS_GuardCondition_get_trigger_value(
    const DDS_GuardCondition* conditon);

/**
 * @fn  DDS_ReturnCode_t DDS_GuardCondition_set_trigger_value(DDS_GuardCondition* conditon, DDS_Boolean value);
 *
 * @ingroup CInfrastruct
 *
 * @brief   �ֶ����ü����Ĵ���״̬��
 *
 * @param [in,out]  conditon    ָ��Ŀ�ꡣ
 * @param   value               true��ʾ����Ϊ����״̬��false��ʾ����Ϊ�Ǵ���״̬��
 *
 * @return  ���Ƿ��� #DDS_RETCODE_OK ��
 */

DCPSDLL DDS_ReturnCode_t DDS_GuardCondition_set_trigger_value(
    DDS_GuardCondition* conditon, 
    DDS_Boolean value);

#endif /* _ZRDDS_INCLUDE_GUARD_CONDITION */

#ifdef _ZRDDS_INCLUDE_READ_CONDITION

/**
 * @fn  DCPSDLL DDS_Boolean DDS_ReadCondition_get_trigger_value(const DDS_ReadCondition* conditon);
 *
 * @ingroup CSubscription
 *
 * @brief   ��ȡ��ȡ������ǰ�Ĵ���״̬��
 *
 * @param   conditon    ָ��Ŀ�ꡣ
 *
 * @return  true��ʾ��������false��ʾ��������δ��������
 */

DCPSDLL DDS_Boolean DDS_ReadCondition_get_trigger_value(
    const DDS_ReadCondition* conditon);

/**
 * @fn  DCPSDLL DDS_DataReader* DDS_ReadCondition_get_datareader(const DDS_ReadCondition* condition);
 *
 * @ingroup CSubscription
 *
 * @brief   ��ȡָ����ȡ�������������ݶ��ߡ�
 *
 * @param   condition   ָ��Ŀ�ꡣ
 *
 * @return  ָ�򴴽��ö�ȡ���������ݶ��߶���
 */

DCPSDLL DDS_DataReader* DDS_ReadCondition_get_datareader(
    DDS_ReadCondition* condition);

/**
 * @fn  DCPSDLL DDS_SampleStateMask DDS_ReadCondition_get_sample_state_mask(const DDS_ReadCondition* condition);
 *
 * @ingroup CSubscription
 *
 * @brief   ��ȡ����ʱ���������״̬���롣
 *
 * @param   condition   ָ��Ŀ�ꡣ
 *
 * @return  �ö�ȡ��������״̬���롣
 */

DCPSDLL DDS_SampleStateMask DDS_ReadCondition_get_sample_state_mask(
    const DDS_ReadCondition* condition);

/**
 * @fn  DCPSDLL DDS_ViewStateMask DDS_ReadCondition_get_view_state_mask(const DDS_ReadCondition* condition);
 *
 * @ingroup CSubscription
 *
 * @brief   ��ȡ����ʱ�������ͼ״̬���롣
 *
 * @param   condition   ָ��Ŀ�ꡣ
 *
 * @return  �ö�ȡ������ͼ״̬���롣
 */

DCPSDLL DDS_ViewStateMask DDS_ReadCondition_get_view_state_mask(
    const DDS_ReadCondition* condition);

/**
 * @fn  DCPSDLL DDS_InstanceStateMask DDS_ReadCondition_get_instance_state_mask(const DDS_ReadCondition* condition);
 *
 * @ingroup CSubscription
 *
 * @brief   ��ȡ����ʱ�����ʵ��״̬���롣
 *
 * @param   condition   ָ��Ŀ�ꡣ
 *
 * @return  �ö�ȡ����ʵ��״̬���롣
 */

DCPSDLL DDS_InstanceStateMask DDS_ReadCondition_get_instance_state_mask(
    const DDS_ReadCondition* condition);

#endif /* _ZRDDS_INCLUDE_READ_CONDITION */

/**
 * @typedef Condition* ConditionPtr
 *
 * @ingroup CInfrastruct
 *
 * @brief   ���� DDS_Condition ָ��ı�����
 */

typedef DDS_Condition* ConditionPtr;

/**
 * @struct DDS_ConditionSeq 
 *
 * @ingroup CInfrastruct
 *
 * @brief   ���� ConditionPtr ���������ͣ��μ� #DDS_USER_SEQUENCE_C ��
 */

DDS_SEQUENCE_C(DDS_ConditionSeq, ConditionPtr);

#ifdef __cplusplus
}
#endif

#endif /* Condition_h__*/
