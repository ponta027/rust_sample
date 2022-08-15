
import unittest
import requests
import json
import logging

logging.basicConfig(format='%(levelname)s:%(message)s', level=logging.INFO)

class TestTodo(unittest.TestCase):

    max_id = 0
    TEST_MESSAGE = "test message"

    def tearDown(self):
        self._delete_all()


    def setUp(self):
        for i in range(0,10):
            response = requests.post('http://localhost:9000/json', json={'message': self.TEST_MESSAGE+str(i) })
            self.assertEqual(response.status_code,200)
            self.assertEqual(json.loads(response.text)["status"],"ok")

    def _max(self, data):
        id = 0
        for d in data:
            if id < d["id"] : 
                id = d["id"]
        return id

    

    def test_delete(self):
        url = self.createUrl("{}".format("all"))
        resp = requests.get(url)
        json_data = json.loads(resp.text)
        max_id = self._max(json_data)
        max_id = 9

        url = self.createUrl("{}/del".format(max_id))
        response = requests.get(url)
        self.assertEqual(response.status_code,200)
        self.assertEqual(json.loads(response.text)["status"],"ok")
 
        url = self.createUrl("{}".format(max_id))
        response = requests.get(url)
        self.assertEqual(response.status_code,404)


    def test_update(self):
        url = self.createUrl("{}".format("all"))
        resp = requests.get(url)
        json_data = json.loads(resp.text)
        max_id = self._max(json_data)

        url = self.createUrl("{}".format(max_id))
        response = requests.post(url , json={'message': self.TEST_MESSAGE })

        self.assertEqual(response.status_code,200)
        self.assertEqual(json.loads(response.text)["status"],"ok")
 
    def test_new(self):
        url = self.createUrl("{}".format("all"))
        resp = requests.get(url)
        json_data = json.loads(resp.text)
        data_len = (len(json_data))

        url = self.createUrl("{}".format(""))
        response = requests.post(url, json={'message': self.TEST_MESSAGE })

        self.assertEqual(response.status_code,200)
        self.assertEqual(json.loads(response.text)["status"],"ok")

        max_id = json.loads(response.text)["id"]

        url = self.createUrl("{}".format(max_id))
        resp = requests.get(url)
        json_data = json.loads(resp.text)

        self.assertEqual(json_data["id"],max_id)
        self.assertEqual(json_data["message"],self.TEST_MESSAGE )


    def _delete_all(self):
        url = self.createUrl("{}".format("all"))
        resp = requests.get(url)
        self.assertEqual(resp.status_code,200)
        json_data = json.loads(resp.text)
        for d in json_data:
            self.delete(d["id"])

    def delete(self , index ):
        url = self.createUrl("{}/del".format(index))
        response = requests.get(url)
        return response

    def createUrl( self , path ):
        url = "http://localhost:9000/json/{}".format(path)
        return url
if __name__ == "__main__":

    unittest.main()
