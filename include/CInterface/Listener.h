/**
 * @file:       Listener.h
 *
 * copyright:   Copyright (c) 2018 ZhenRong Technology, Inc. All rights reserved.
 */

#ifndef Listener_h__
#define Listener_h__

/**
 * @struct DDS_Listener
 *
 * @ingroup CListener
 *
 * @brief   ����ʵ����������͵ġ����ࡱ��
 */

typedef struct DDS_Listener
{
    /** @brief   �������ݣ��������յ�Listener�� */
    void* user_data;
}DDS_Listener;

#endif /* Listener_h__*/
