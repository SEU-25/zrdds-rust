/**
 * @file:       WaitSet.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef WaitSet_h__
#define WaitSet_h__

#include "ZRDDSCommon.h"
#include "ReturnCode_t.h"
#include "Condition.h"
#include "Duration_t.h"

#ifdef _ZRDDS_INCLUDE_WAITSET

#ifdef __cplusplus
extern "C"
{
#endif

/**
 * @fn  DCPSDLL DDS_WaitSet* DDS_WaitSet_new();
 *
 * @ingroup CInfrastruct
 *          
 * @brief   ����һ���µĿյĵȴ����ϡ�
 *
 * @return  NULL��ʾʧ�ܣ���NULL�ı�ʾ�����ĵȴ����ϡ�
 */

DCPSDLL DDS_WaitSet* DDS_WaitSet_new();

/**
 * @fn  DCPSDLL void DDS_WaitSet_delete(DDS_WaitSet* waitset);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ɾ��ָ���ĵȴ����ϣ����Զ��⿪������
 *
 * @param [in,out]  waitset        ָ��Ŀ�ꡣ
 */

DCPSDLL void DDS_WaitSet_delete(DDS_WaitSet* waitset);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_WaitSet_attach_condition(DDS_WaitSet* self, DDS_Condition* condition);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ��ָ�����������뵽�õȴ������С�
 *
 * @details �����ӵ������Ĵ���״̬Ϊtrue������⿪������
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param [in,out]  condition ָ����Ҫ��ӵ�������
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ӳɹ���
 *          - #DDS_RETCODE_OUT_OF_RESOURCES :�����ڴ�ʧ�ܣ�
 *          - #DDS_RETCODE_BAD_PARAMETER :����������Ч��������
 */

DCPSDLL DDS_ReturnCode_t DDS_WaitSet_attach_condition(
    DDS_WaitSet* self, 
    DDS_Condition* condition);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_WaitSet_detach_condition(DDS_WaitSet* self, DDS_Condition* condition);
 *
 * @ingroup CInfrastruct
 *
 * @brief   �ӵȴ��������Ƴ�ָ����������
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param [in,out]  condition ָ����Ҫɾ����������
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :��ӳɹ���
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :ָ�����������ڸõȴ������У�
 *          - #DDS_RETCODE_BAD_PARAMETER :����������Ч��������
 */

DCPSDLL DDS_ReturnCode_t DDS_WaitSet_detach_condition(
    DDS_WaitSet* self, 
    DDS_Condition* condition);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_WaitSet_wait(DDS_WaitSet* self, DDS_ConditionSeq* activeConditions, const DDS_Duration_t* timeout);
 *
 * @ingroup CInfrastruct
 *
 * @brief   �����ȴ��ȴ������е��������ڴ���״̬��
 *
 * @details Ӧ���̵߳ȴ�ĳЩ�����ķ��������������̣߳� �����óɹ�ʱ��@e activeConditions �д�Ŵ�����������
 *          ���������̵߳ȴ�ͬһ���ȴ�������
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param   activeConditions    ���ڲ��������ڴ洢���ڴ���״̬��������
 * @param   timeout             ָ����ȴ�ʱ�䡣
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :���ڲ�����Ч���������ڴ���״̬��������
 *          - #DDS_RETCODE_PRECONDITION_NOT_MET :�ڶ���߳��ϵȴ�������
 *          - #DDS_RETCODE_BAD_PARAMETER / #DDS_RETCODE_ERROR ���û��ṩ�Ŀռ䲻�㣬�ҵײ�����ʧ�ܣ�
 *          - #DDS_RETCODE_TIMEOUT : ��ָ����ʱ����û���������ڴ���״̬��
 */

DCPSDLL DDS_ReturnCode_t DDS_WaitSet_wait(
    DDS_WaitSet* self, 
    DDS_ConditionSeq* activeConditions, 
    const DDS_Duration_t* timeout);

/**
 * @fn  DCPSDLL DDS_ReturnCode_t DDS_WaitSet_get_conditions(DDS_WaitSet* self, DDS_ConditionSeq* attachedConditions);
 *
 * @ingroup CInfrastruct
 *
 * @brief   ��ȡ��ǰ�õȴ����������е�������
 *
 * @param [in,out]  self        ָ��Ŀ�ꡣ
 * @param [in,out]  attachedConditions ���ڲ��������ڴ洢��ǰ�����е�������
 *
 * @return  ��ǰ���ܵķ���ֵ��
 *          - #DDS_RETCODE_OK :���ڲ�����Ч���������ڴ���״̬��������
 *          - #DDS_RETCODE_BAD_PARAMETER / #DDS_RETCODE_ERROR ���û��ṩ�Ŀռ䲻�㣬�ҵײ�����ʧ�ܣ�
 */

DCPSDLL DDS_ReturnCode_t DDS_WaitSet_get_conditions(
    DDS_WaitSet* self, 
    DDS_ConditionSeq* attachedConditions);

#ifdef __cplusplus
}
#endif

#endif /* _ZRDDS_INCLUDE_WAITSET */

#endif /* WaitSet_h__*/
