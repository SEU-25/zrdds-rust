/**
 * @file:       PublisherListener.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef PublisherListener_h__
#define PublisherListener_h__

#include "DataWriterListener.h"

/**
 * @struct DDS_PublisherListener
 *
 * @ingroup CListener
 *
 * @brief   �����߼��������͡�
 */

typedef struct DDS_PublisherListener
{
    /** @brief   �ó�Ա���ڼ��������ߵ�����д����ʵ���״̬�� */
    DDS_DataWriterListener datawriter_listener;
}DDS_PublisherListener;

/**
 * @def DDS_PublisherListener_initial(listener)
 *
 * @ingroup CListener
 *
 * @brief   ��ʼ�������߼���������Ϊ���г�ԱΪ�ա�
 *
 * @param   listener    ��Ҫ��ʼ���ķ����߼���������
 */

#define DDS_PublisherListener_initial(listener) memset(listener, 0, sizeof(DDS_PublisherListener))

#endif /* PublisherListener_h__*/
